import { derived } from "svelte/store"
import Submission from "./submission";
import Source from "./source";

const store = derived([Submission, Source], ([flakes, source], set) => {

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

    }
  }

  const ready = source.datasets.filter(
    d => {
      const validated = datasets[`${d.name}`];
      if (!d.metadata || !validated || Object.keys(validated.extras.errors).length) { return false }

      return d.resources.every(
        r => {
          const validated = resources[`${d.name}/${r.name}`];
          return r.metadata && validated && !Object.keys(validated.extras.errors).length
        }
      )
    }
  ).map(d => d.name)
  set({
    uploads,
    datasets,
    resources,
    ready
  });
}, { uploads: {}, datasets: {}, resources: {}, ready: [] })

export default store
