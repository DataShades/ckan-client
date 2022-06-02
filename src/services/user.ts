import { writable, get } from "svelte/store"
import type { TUser } from "../types";
import Project from './project'
import Tauri from './tauri'
import Toaster from "./toaster"
import User from "./user"
const { subscribe, set } = writable<TUser | null>(null)

const resolve = async portal => {
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
    restore(user)
    return user
  } catch (e) {
    Toaster.error(e, "Login failed");
  }
}

const restore = async (user: TUser) => {
  set(user)
  await Project.restore(user);
}

const logout = () => {
  set(null);
}

export default {
  subscribe,
  resolve,
  logout,
};
