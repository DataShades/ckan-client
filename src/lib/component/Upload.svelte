<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { TDataset, TResource } from "src/types";

  import { Button, Icon, Progress } from "sveltestrap";

  import { Flakes } from "../../services";
  import { humanizeSize } from "../../utils";

  export let details: any;
  export let queued: boolean;
  export let resource: TResource;
  export let dataset: TDataset;

  const dispatch = createEventDispatcher();

  let pending = false;

  let validated = false;

  Flakes.subscribe(({ datasets, resources }) => {
    const d = datasets[`${dataset.name}`];
    const r = resources[`${dataset.name}/${resource.name}`];
    validated =
      d &&
      r &&
      !Object.keys(d.extras.errors).length &&
      !Object.keys(r.extras.errors).length;
  });
  let progress = details ? (
    (details.data.bytes_uploaded / (details.data.size + 1)) *
    100
  ).toFixed(0) : 0;
</script>

<div class="item-inner">
  {resource.name}
  <Button
    disabled={!queued}
    class="float-end"
    title="Pause upload"
    color="link"
    on:click={() => dispatch("pause", { dataset, resource })}
  >
    <Icon name="pause-circle-fill" />
  </Button>

  <Button
    color="primary"
    class="float-end"
    outline
    disabled={!validated ||
      !resource.metadata ||
      pending ||
      (details && details.data.completed) ||
      queued}
    on:click={() => dispatch("upload", { dataset, resource })}
    title="Start/resume upload"
  >
    <Icon name="cloud-upload-fill" />
    Upload ({humanizeSize(resource.size)})
  </Button>
</div>
{#if details}
  <div class:opacity-75={(details && details.data.completed) || !queued}>
    <Progress color="primary" value={progress}>
      {progress}%
    </Progress>
  </div>
{/if}
