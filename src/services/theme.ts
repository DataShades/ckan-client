import {writable} from "svelte/store";

const {subscribe, update} = writable("sketchy")

const next = () => {

  update((current: string) => {
    const idx = themes.findIndex(v => v === current);
    const newIdx = (idx + 1) % themes.length;
    return themes[newIdx];
  })
}

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
  subscribe, next
}
