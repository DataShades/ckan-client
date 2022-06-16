import { writable, get } from "svelte/store"
import Toaster from "./toaster"
import Tauri from "./tauri"
import Source from "./source"

const store = writable([]);


const refresh = async () => {
  try {
    store.set(await Tauri.invoke("show_submission"))
  } catch (e) {
    Toaster.error(e, "Error");
    store.set([])
  }
}

const registerUpload = async (dataset: string, name: string) => {
  await Tauri.invoke("register_upload", { path: get(Source).path, dataset, name }).catch(e => Toaster.error(e, `[${dataset}] ${name}`))
  await refresh();
}

const
finalize = async () => {
  try{
    await Tauri.invoke("submission_finalize")
    Toaster.info("Check you mail box", "Submission completed")
  } catch(err){
    Toaster.error(err, "Creation failed")
  }


}

const progressUpload = async (dataset: string, name: string, part: number) => {
  try {
    await Tauri.invoke("progress_upload", { path: get(Source).path, dataset, name, part })
  } catch (e) {
    Toaster.error(e, "Error")
  }
  await refresh();
}

const validateDataset = async (name: string) => {
  const source = get(Source)
  if (source) {
    await Tauri.invoke("validate_dataset", { path: source.path, name }).catch(e => Toaster.error(e, name))
    await Source.change(source.path);
    refresh();
  }

}

const validateResource = async (dataset: string, name: string) => {
  const source = get(Source)
  if (source) {
    await Tauri.invoke("validate_resource", { path: source.path, dataset, name }).catch(e => Toaster.error(e, `[${dataset}] ${name}`))
    await Source.change(source.path);
  }
}


const byType = (type: string) => get(store).filter(i => i.extras.type === type);

export default {
  subscribe: store.subscribe,
  refresh,
  byType,
  registerUpload,
  progressUpload,
  validateDataset,
  validateResource,
  finalize
}
