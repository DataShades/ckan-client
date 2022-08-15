import { writable } from "svelte/store";
import Toaster from "./toaster";
import type { TProject, TUser } from "../types";
import Storage from "./storage";
import Tauri from "./tauri"

const { subscribe, set } = writable<TProject | null>(null);
const filter = async (name: string): Promise<TProject[]> => {
  return await Tauri.invoke("list_projects", {name: name || ""})
}

const restore = async (user: TUser) => {
  const name: string | null = await Storage.getItem(`user:${user.id}:project`)
  if (!name) { return }
  const projects = await filter(name);
  set(projects.find(p => p.name === name))
}

const choose = (project: TProject|null, user: TUser) => {
  set(project)
    Storage.setItem(`user:${user.id}:project`, project && project.name);
    Tauri.invoke("project_set", {"id": project && project.id}).catch(err => Toaster.error(err, "Error"))

}

const reset = (user: TUser) => {
  choose(null, user)
}

export default {
  filter,
  subscribe,
  choose,
  reset,
  restore,
}
