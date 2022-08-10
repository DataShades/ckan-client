import { writable , get} from "svelte/store"
import type { TUser, TPortal } from "../types";
import Project from './project'
import Source from './source'
import Submission from './submission'
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
    const user: TUser = await Tauri.invoke("login", { portal })
    await restore(user)
    return user
  } catch (e) {
    console.log(e)
    Toaster.error(e, "Login failed");
  }
}

const restore = async (user: TUser) => {
  await Project.restore(user);
  await Source.restore(user);
  set(user)
  await Submission.refresh();
}

const logout = () => {
  set(null);
  console.log("Logged out")
}

export default {
  subscribe,
  resolve,
  logout,
};
