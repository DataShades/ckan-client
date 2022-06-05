<script lang="ts">
  import {
    Alert,
    Button,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    CardSubtitle,
    CardText,
    CardTitle,
  } from "sveltestrap";

  import { Source, Toaster } from "../../services";

  const save = async (name: string) => {
    try {
      await Source.saveDataset(name);
    } catch (e) {
      Toaster.error(e, "Error");
    }
  };
</script>


<div class="m-5">
  {#if !$Source.datasets.length}
    <Alert color="primary">The source contains no datasets.</Alert>
  {:else}
    {#each $Source.datasets as dataset}
      <Card class="mb-3">
        <CardHeader>
          <CardTitle>{dataset.name}</CardTitle>
        </CardHeader>
        <CardBody>
          {#if dataset.metadata}
            <CardSubtitle>Metadata</CardSubtitle>
            <CardText>
              <pre>{JSON.stringify(dataset.metadata, null, 2)}</pre>
            </CardText>
          {:else}
            <CardText>
              This dataset does not have metadata file. Click
              <Button on:click={() => save(dataset.name)}>this button</Button>
              in order to create it.
            </CardText>
          {/if}
        </CardBody>
        <CardFooter>
          <Button>Button</Button>
        </CardFooter>
      </Card>
    {/each}
  {/if}
</div>
