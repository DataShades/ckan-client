import {derived} from "svelte/store"
import Submission from "./submission";
import Source from "./source";

const store = derived([Submission, Source], ([flakes], set) => {

  const uploads = {};
  for (let item of flakes) {
    if (item.extras.type === "client-upload") {
      uploads[`${item.data.dataset}/${item.data.name}`] = item;
    }
  }

  set({
    uploads,
  });
}, {uploads: {}})

export default store
