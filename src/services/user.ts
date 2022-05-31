import {writable} from "svelte/store"

const {subscribe, set} = writable(null)
const resolve = async portal => {
  // FIXME: put real implementation here
  await new Promise(c => setTimeout(c, 200))
  if (!portal.token) return null

  const user = {
    fullname: portal.url,
  }
  set(user);
  return user;
}

const logout = () => {
  set(null);
}

export default {
  subscribe,
  resolve,
  logout,
};
