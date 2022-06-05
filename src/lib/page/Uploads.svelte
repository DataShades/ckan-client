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

  const save = async (dataset: string, name: string) => {
    try {
      await Source.saveResource(dataset, name);
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
      {#if dataset.resources.length}
          <Card class="mb-3">
            <CardHeader>
              <CardTitle>{dataset.name}</CardTitle>
            </CardHeader>
            <CardBody>
        {#each dataset.resources as resource}
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
                  <Button on:click={() => save(dataset.name, resource.name)}>
                    this button
                  </Button>
                  in order to create it.
                </CardText>
              {/if}
            </CardBody>
            <CardFooter>
              <Button>Button</Button>
            </CardFooter>
          </Card>
        {/each}
            </CardBody>
            <CardFooter>
              <Button>Button</Button>
            </CardFooter>
          </Card>

      {/if}
    {/each}
  {/if}
</div>
