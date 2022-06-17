import type { TDataset, TResource } from "../types";
import { writable, get } from "svelte/store";

import Source from "./source";
import Flakes from "./flakes";
import Toaster from "./toaster";
import Submission from "./submission";

const store = writable({
  items: new Map<string, [TDataset, TResource]>(),
  processing: false,
});

const add = (dataset: TDataset, resource: TResource) => {
  store.update((queue) => {
    queue.items.set(`${dataset.name}/${resource.name}`, [dataset, resource]);
    return queue;
  });
};

const drop = (dataset: TDataset, resource: TResource) => {
  store.update((queue) => {
    queue.items.delete(`${dataset.name}/${resource.name}`);
    return queue;
  });
};

const clear = () => store.update((queue) => ({ ...queue, items: new Map() }));

const pop = () => {
  let value: [TDataset, TResource] | null;
  store.update((queue) => {
    let entry = queue.items.entries().next().value;
    if (entry) {
      value = entry[1];
      queue.items.delete(entry[0]);
    }
    return queue;
  });
  return value;
};
const contains = (dataset: TDataset, resource: TResource) => {
  return get(store).items.has(`${dataset.name}/${resource.name}`);
};

const process = async () => {
  let resolve, reject;
  const handle = new Promise((suc, err) => {
    resolve = suc;
    reject = err;
  });
  store.update((queue) => {
    if (queue.processing) {
      reject();
      return queue;
    }
    resolve();
    return { ...queue, processing: true };
  });
  await handle.catch(() => console.log("Join existing processing"));
  try {
    await exhaustQueue();
  } catch (err) {
    Toaster.error(err, "Upload error");
    clear();
  }
  store.update((queue) => ({ ...queue, processing: false }));
};

const fullUpload = async (finalize: boolean) => {
  const uploads = get(Flakes).uploads;
  get(Source).datasets.forEach((d) =>
    d.resources
      .filter((r) => {
        const upload = uploads[`${d.name}/${r.name}`];
        return !upload || !upload.data.completed;
      })
      .forEach((r) => add(d, r))
  );
  await process();
  if (finalize) {
    await Submission.finalize()
  }

};

const exhaustQueue = async () => {
  while (true) {
    let key = pop();
    if (!key) {
      break;
    }
    const [dataset, resource] = key;
    const upload = get(Flakes).uploads[`${dataset.name}/${resource.name}`];
    if (!upload) {
      Toaster.info(`Prepare ${resource.name} for upload`, dataset.name);
      add(dataset, resource);
      await Submission.registerUpload(dataset.name, resource.name);
    } else if (!upload.data.completed) {
      add(dataset, resource);
      await Submission.progressUpload(
        dataset.name,
        resource.name,
        upload.data.chunks_uploaded + 1
      );
    } else {
      Toaster.info(`Completed uploading ${resource.name}`, dataset.name);
    }
  }
};

export default {
  subscribe: store.subscribe,
  add,
  drop,
  pop,
  contains,
  process,
  fullUpload,
  clear,
};
