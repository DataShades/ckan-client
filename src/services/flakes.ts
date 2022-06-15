import {derived} from "svelte/store"
import Submission from "./submission";
import Source from "./source";

const store = derived([Submission, Source], ([flakes], set) => {

  const uploads = {};
  const datasets = {};
  const resources = {};

  for (let item of flakes) {
    switch (item.extras.type) {
      case "client-upload":
        uploads[`${item.data.dataset}/${item.data.name}`] = item;
        break;
      case "validated-dataset":
        datasets[`${item.extras.dataset}`] = item;
        break;
      case "validated-resource":
        resources[`${item.extras.dataset}/${item.extras.resource}`] = item;
        break;
      default:
        // console.log(item.extras.type)
    }
  }

  set({
    uploads,
    datasets,
    resources,
  });
}, {uploads: {}, datasets: {}, resources: {}})

export default store
