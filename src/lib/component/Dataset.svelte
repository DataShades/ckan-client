<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { TDataset } from "src/types";
  import { humanizeSize } from "../../utils";
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
  import { Source, Flakes, Queue } from "../../services";
  import { Resource } from ".";
  import NiceMetadata from "./NiceMetadata.svelte";

  const dispatch = createEventDispatcher();

  export let dataset: TDataset;
  export let validated: any | null;
  export let pending: boolean;

  $: size = humanizeSize(
    dataset.resources.reduce((total, r) => total + (r.size || 0), 0)
  );
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
                    upload a dataset without it. This app can create metadata,
                    but you should change some data there in the next stage.
                  </p>
                  <Button
                    color="primary"
                    on:click={() => Source.saveDataset(dataset.name)}
                  >
                    Create metadata automatically
                  </Button>
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
            <Resource
              {resource}
              {dataset}
            />
          {/each}
        </CardText>
      </AccordionItem>
      <Card class="mt-4">
        <CardHeader>
          <CardTitle>Validation Errors</CardTitle>
        </CardHeader>
        <CardBody>
          <CardText>
            {#if !dataset.metadata}
              <Alert color="danger">Create metadata to validate it.</Alert>
            {:else if !validated}
              <Alert color="warning">Nothing validated yet.</Alert>
            {:else if validated.extras.errors.length}
              {#each validated.extras.errors as error}
              <Alert color="danger">

              <pre>{JSON.stringify(error, null, 2)}</pre>
              </Alert>
              {/each}
            {:else}
              <Alert color="success">All files are valid.</Alert>
            {/if}
          </CardText>
        </CardBody>
      </Card>
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

    <Button
      color="primary"
      class="ms-2"
      disabled={!dataset.metadata ||
        uploading >= dataset.resources.length ||
        !$Flakes.ready.includes(dataset.name)}
      on:click={() => dispatch("upload", { dataset })}
    >
      Upload({size})
    </Button>
  </CardFooter>
</Card>
