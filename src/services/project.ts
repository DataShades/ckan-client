import { writable } from "svelte/store";
import Toaster from "./toaster";
import type { TProject, TUser } from "../types";
import Storage from "./storage";
import Tauri from "./tauri"

const { subscribe, set } = writable<TProject | null>(null);

let projects: TProject[] = [
  { id: "1", name: 'proj-1', title: "Project" },
  { id: "2", name: 'proj-2', title: "Hello" },
  { id: "3", name: 'proj-3', title: "World" },
  { id: "4", name: 'proj-4', title: "and" },
  { id: "5", name: 'proj-5', title: "Hello World" },
  { id: "6", name: 'proj-6', title: "another proj" },
  { id: "7", name: 'proj-7', title: "no-no" },
  { id: "8", name: 'proj-8', title: "and final" },
]

const filter = async (name: string): Promise<TProject[]> => {
  return await Tauri.invoke("list_projects", {name: name || ""})

  if (!name) {
    return projects
  }

  return projects.filter(p => ~p.name.indexOf(name) || ~p.title.indexOf(name))
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
