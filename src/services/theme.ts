import {writable} from "svelte/store";

const {subscribe, update} = writable("sketchy")

const change = (shift: number) => {
  update((current: string) => {
    const idx = themes.findIndex(v => v === current);
    const newIdx = (idx + shift) % themes.length;
    return themes[newIdx];
  })
}

const next = () => change(1);
const prev = () => change(themes.length-1);

const themes = [
  "cerulean",
  "cosmo",
  "cyborg",
  "darkly",
  "flatly",
  "journal",
  "litera",
  "lumen",
  "lux",
  "materia",
  "minty",
  "morph",
  "pulse",
  "quartz",
  "sandstone",
  "simplex",
  "sketchy",
  "slate",
  "solar",
  "spacelab",
  "superhero",
  "united",
  "vapor",
  "yeti",
  "zephyr",
]

export default {
  subscribe, next, prev
}
