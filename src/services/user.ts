import { writable } from "svelte/store"
import type { TUser,TPortal } from "../types";
import Project from './project'
import Source from './source'
import Tauri from './tauri'
import Toaster from "./toaster"

const { subscribe, set } = writable<TUser | null>(null)

const resolve = async (portal: TPortal) => {
  if (!portal.url) {
    Toaster.error("URL cannot be empty", "Login rejected");
    return
  }

  if (!portal.token) {
    Toaster.error("Token cannot be empty", "Login rejected");
    return
  }

  try {
    const user: TUser = await Tauri.invoke("login", {portal})
    await restore(user)
    return user
  } catch (e) {
    Toaster.error(e, "Login failed");
  }
}

const restore = async (user: TUser) => {
  await Project.restore(user);
  await Source.restore(user);
  set(user)

}

const logout = () => {
  set(null);
}

export default {
  subscribe,
  resolve,
  logout,
};
