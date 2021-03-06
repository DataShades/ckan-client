
import { writable } from "svelte/store";
import Storage from "./storage";
import User from "./user"

type PortalDetails = {
  url: string,
  token: string,
}
const KEY = "ckan:portal"
const defaultFactory = () => ({ url: "", token: "" });

const { subscribe, set, update, } = writable(defaultFactory());

const persist = async (portal: PortalDetails) => {
  set(await Storage.setItem(KEY, portal));
}

Storage.getItem(KEY).then(portal => {
  set(portal as PortalDetails | null || defaultFactory());
})

export default {
  subscribe,
  set,
  update,
  persist,
}
