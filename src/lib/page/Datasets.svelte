<script lang="ts">
  import type { TDataset } from "src/types";

  import { Alert, Button, FormGroup, Input, InputGroup } from "sveltestrap";

  import { Source, Flakes, Submission, Queue } from "../../services";
  import { Dataset } from "../component";
  import DatasetsProgress from "../component/DatasetsProgress.svelte";

  let newDatasetName = "";

  let pending: string | null = null;

  const validate = async (e: CustomEvent) => {
    const dataset = e.detail.dataset;
    pending = dataset.name;
    await Submission.validateDataset(dataset.name);
    for (let resource of dataset.resources) {
      await Submission.validateResource(dataset.name, resource.name);
    }
    pending = null;
  };
  const upload = async (e: CustomEvent) => {
    const dataset: TDataset = e.detail.dataset;
    dataset.resources.forEach((r) => Queue.add(dataset, r));
    Queue.process();
  };
</script>

<div class="m-5">
  {#if !$Source.datasets.length}
    <Alert color="primary">The source contains no datasets.</Alert>
  {:else}
    {#each $Source.datasets as dataset}
      <Dataset
        {dataset}
        validated={$Flakes.datasets[dataset.name]}
        pending={pending === dataset.name}
        on:validate={validate}
        on:upload={upload}
      />
    {/each}
  {/if}

  <FormGroup class="mt-5">
    <InputGroup>
      <Input placeholder="Dataset name" bind:value={newDatasetName} />
      <Button on:click={() => Source.addDataset(newDatasetName)}>Create</Button>
    </InputGroup>
  </FormGroup>
</div>

<DatasetsProgress
  total={$Source.datasets.length}
  empty={$Source.datasets.filter((d) => !d.metadata).length}
  ready={$Flakes.ready.length}
/>
