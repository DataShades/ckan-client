<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { TDataset, TResource } from "src/types";

  import { Button, ButtonGroup, Icon, Progress } from "sveltestrap";

  import { Flakes } from "../../services";

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
</script>

{resource.name}
<ButtonGroup size="sm" class="float-end">
  <Button
    disabled={!validated ||
      !resource.metadata ||
      pending ||
      (details && details.data.completed) ||
      queued}
    on:click={() => dispatch("upload", { dataset, resource })}
    title="Start/resume upload"
  >
    <Icon name="cloud-upload" />
    start/resume
  </Button>

  <Button
    disabled={!queued}
    title="Pause upload"
    on:click={() => dispatch("pause", { dataset, resource })}
  >
    <Icon name="pause-circle" />
    pause
  </Button>
</ButtonGroup>

{#if details}
  <hr />
  <Progress
    striped
    animated={pending}
    color={details.data.completed ? "success" : "info"}
    value={((details.data.bytes_uploaded + 1) / (details.data.size + 1)) * 100}
  />
{/if}
