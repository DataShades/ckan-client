<script lang="ts">
  import type { TDataset } from "src/types";
  import {
    Alert,
    Button,
    Card,
    CardBody,
    CardHeader,
    CardText,
    CardTitle,
  } from "sveltestrap";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  export let dataset: TDataset;
  export let validated: any | null;
</script>

<Card class="mb-3 dataset-card bordered">
  <CardHeader>
    <CardTitle>
      {dataset.name}
      <Button
        on:click={() => dispatch("scrollToDataset")}
        class="float-end btn-link"
      >
        Scroll to Dataset
      </Button>
    </CardTitle>
  </CardHeader>
  <CardBody>
    <CardText>
      {#if !dataset.metadata}
        <Alert color="danger">Create metadata to validate it.</Alert>
      {:else if !validated}
        <Alert color="warning">Not validated yet.</Alert>
      {:else if Object.keys(validated.extras.errors).length}
        {#each Object.entries(validated.extras.errors) as [field, errors]}
          <Alert color="danger">
            <span class="badge bg-danger bg-opacity-25 text-black mb-2"
              >Dataset metadata</span
            ><br />
            {dataset.name} |
            <strong>
              {field}
            </strong>:
            {errors.join(";")}
          </Alert>
        {/each}
      {:else}
        <Alert color="success">All files are valid.</Alert>
      {/if}
    </CardText>
  </CardBody>
</Card>
