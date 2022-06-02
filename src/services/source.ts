
import { writable } from "svelte/store"
import Storage from "./storage"
import Tauri from "./tauri"

const key = "submission:storage"

let content = {};

const { subscribe, set } = writable<string>("")

const checkPath = async (path: string) => {
  return await Tauri.invoke("read_source_path", {path})
}

const change = async (path: string) => {
  content = await checkPath(path);

  console.log(content)

  await Storage.setItem(key, path)
  set(path)
  return content;
}

Storage.getItem(key).then((path: string | null) => set(path || ""))

export default {
  subscribe,
  change,
}
