<script lang="ts">
  import {
    Alert,
    Button,
    FormGroup,
    Input,
    InputGroup,
  } from "sveltestrap";

  import { Source, Flakes } from "../../services";
  import {Dataset} from "../component";

  let newDatasetName = "";
</script>

<div class="m-5">
  {#if !$Source.datasets.length}
    <Alert color="primary">The source contains no datasets.</Alert>
  {:else}
    {#each $Source.datasets as dataset}
      <Dataset {dataset} validated={$Flakes.datasets[dataset.name]} />
    {/each}
  {/if}

  <FormGroup class="mt-5">
    <InputGroup>
      <Input placeholder="Dataset name" bind:value={newDatasetName} />
      <Button on:click={() => Source.addDataset(newDatasetName)}>Create</Button>
    </InputGroup>
  </FormGroup>
</div>
