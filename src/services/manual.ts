import { writable } from "svelte/store"

const { set, update, subscribe } = writable({ open: false, active: null })

const toggle = (active) => update(manual => ({ active, open: !manual.open, }))
const setActive = (active) => update(manual => ({ ...manual, active }))
export default { set, update, subscribe, toggle, setActive }
