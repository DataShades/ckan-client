<script lang="ts">
  import {
    Button,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    CardSubtitle,
    CardText,
    CardTitle,
  } from "sveltestrap";
  import type { TResource, TDataset } from "../../types";
  import { Source } from "../../services";

  export let resource: TResource;
  export let dataset: TDataset;
</script>

<Card class="mb-3">
  <CardHeader>
    <CardTitle>{resource.name}</CardTitle>
  </CardHeader>
  <CardBody>
    {#if resource.metadata}
      <CardSubtitle>Metadata</CardSubtitle>
      <CardText>
        <pre>{JSON.stringify(resource.metadata, null, 2)}</pre>
      </CardText>
    {:else}
      <CardText>
        This resource does not have metadata file. Click
        <Button
          on:click={() => Source.saveResource(dataset.name, resource.name)}
        >
          this button
        </Button>
        in order to create it.
      </CardText>
    {/if}
  </CardBody>
  <CardFooter>
    <Button on:click={() => Source.open(dataset.name, resource.name)}>
      Open file
    </Button>
    {#if resource.metadata}
      <Button
        on:click={() => Source.open(dataset.name, resource.name + ".json")}
      >
        Open metadata
      </Button>
    {/if}
  </CardFooter>
</Card>
