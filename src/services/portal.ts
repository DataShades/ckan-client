
import { navigate } from "svelte-routing";
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

Storage.getItem(KEY).then(async (portal: PortalDetails | null) => {
  console.debug("[BUT] Restore portal details from session: %o", portal);

  set(portal || defaultFactory());
  if (portal && portal.url && portal.token) {
    if (User.resolve(portal)) {
      console.debug("[BUT] User resoved. Navigate to /project")
      navigate("/project")
      // TODO: drop
      navigate("/datasets")
    }
  }
})

export default {
  subscribe,
  set,
  update,
  persist,
}
