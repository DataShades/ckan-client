<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { TDataset } from "src/types";
  import {
    Accordion,
    AccordionItem,
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
  import { Source, Queue } from "../../services";
  import { Resource } from ".";
  import NiceMetadata from "./NiceMetadata.svelte";

  const dispatch = createEventDispatcher();

  export let dataset: TDataset;
  export let validated: any | null;
  export let pending: boolean;

  $: nErrors = validated ? Object.keys(validated.extras.errors).length : 0;

 $: nResourcesWithoutMetadata = dataset.resources.filter(
    (r) => !r.metadata
  ).length;
  let uploading: number = 0;
  Queue.subscribe((q) => {
    uploading = dataset.resources.filter((resource) =>
      Queue.contains(dataset, resource)
    ).length;
  });
</script>

<span id={btoa(dataset.name)}></span>
<Card class="mb-3 dataset-card">
  <CardHeader>
    <CardTitle>
      {dataset.name}
      <Button
        on:click={() => Source.open(dataset.name)}
        class="float-end btn-link"
      >
        Open folder
      </Button>
    </CardTitle>
  </CardHeader>
  <CardBody>
    <Accordion>
      <AccordionItem>
        <div slot="header">
          Dataset Metadata
          {#if !dataset.metadata}
            <span class="badge bg-danger bg-opacity-25 text-black ms-3">
              Metadata is missing
            </span>
          {:else if nErrors}
            <span class="badge bg-danger bg-opacity-25 text-black ms-3">
              {nErrors} error{nErrors > 1 ? "s" : ""}
            </span>
          {/if}
        </div>
        <Card>
          <CardBody>
            <CardText>
              {#if dataset.metadata}
                <NiceMetadata metadata={dataset.metadata} />
              {:else}
                <Alert color="danger">
                  <h4>Metadata is missing</h4>
                  <p>
                    The selected folder doesn’t have metadata. You will not
                    upload a dataset without it.
                  </p>
                </Alert>
              {/if}
            </CardText>
          </CardBody>
          {#if dataset.metadata}
            <CardFooter>
              <Button
                color="primary"
                outline
                on:click={() => Source.open(dataset.name + ".toml")}
              >
                Open metadata in editor
              </Button>
            </CardFooter>
          {/if}
        </Card>
      </AccordionItem>
      <AccordionItem class="dataset-resources">
        <CardSubtitle slot="header">
          Resources
          {#if nResourcesWithoutMetadata}
            <span class="badge bg-danger bg-opacity-25 text-black ms-3">
              {#if nResourcesWithoutMetadata === 1}
                1 resource doesn’t have the metadata
              {:else}
                {nResourcesWithoutMetadata} resources don’t have the metadata
              {/if}
            </span>
          {/if}
        </CardSubtitle>
        <CardText>
          {#each dataset.resources as resource}
            <Resource {resource} {dataset} />
          {/each}
        </CardText>
      </AccordionItem>
    </Accordion>
  </CardBody>
  <CardFooter>
    <Button
      color="primary"
      outline
      disabled={!dataset.metadata || pending}
      on:click={() => dispatch("validate", { dataset })}
    >
      Validate
    </Button>
  </CardFooter>
</Card>
