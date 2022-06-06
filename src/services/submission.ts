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

const swap = (upload: any) => store.update(items => items.map(i => i.id === upload.id ? upload : i))

const registerUpload = async (dataset: string, name: string) => {
  await Tauri.invoke("register_upload", { path: get(Source).path, dataset, name }).catch(e => Toaster.error(e, "Error"))
  await refresh();
}

const progressUpload = async (dataset: string, name: string, part: number) => {
  try {
    await Tauri.invoke("progress_upload", { path: get(Source).path, dataset, name, part })
  } catch (e) {
    Toaster.error(e, "Error")
  }
  await refresh();
}


const byType = (type: string) => get(store).filter(i => i.extras.type === type);

export default {
  subscribe: store.subscribe,
  refresh,
  byType,
  registerUpload,
  progressUpload,
}
