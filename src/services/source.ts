
import type { TSource, TUser } from "src/types";
import { writable, get } from "svelte/store"
import Storage from "./storage"
import Tauri from "./tauri"

const key = "submission:storage"

const defaultSource = () => ({ path: "", metadata: null, datasets: [] });
const { subscribe, set } = writable<TSource>(defaultSource())

const checkPath = async (path: string): Promise<TSource> => {
  return await Tauri.invoke("read_source_path", { path })
}

const change = async (path: string) => {
  const source = path ? await checkPath(path) : defaultSource();

  await Storage.setItem(key, source.path)
  set(source)
  return source
}

const restore = (_user: TUser) => Storage.getItem(key).then((path: string | null) => change(path))

const saveRoot = async () => {
  const source = get(service)
  if (source) {
    await Tauri.invoke("save_root_metadata", {path: source.path, metadata: source.metadata || {"example": "EXAMPLE"}})
    await change(source.path);
  }

}

const saveDataset = async (name: string) => {
  const source = get(service)
  if (source) {
    await Tauri.invoke("save_dataset_metadata", {path: source.path, name, metadata: source.metadata || {"example": "EXAMPLE"}})
    await change(source.path);
  }

}

const saveResource = async (dataset:string, name: string) => {
  const source = get(service)
  if (source) {
    await Tauri.invoke("save_resource_metadata", {path: source.path, dataset, name, metadata: source.metadata || {"example": "EXAMPLE"}})
    await change(source.path);
  }

}


const service = {
  subscribe,
  change,
  restore,
  saveRoot,
  saveDataset,
  saveResource,
}

export default service;
