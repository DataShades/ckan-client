<script lang="ts">
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
  import type { TResource, TDataset } from "../../types";
  import { Source } from "../../services";
  import { NiceMetadata } from ".";

  export let resource: TResource;
  export let dataset: TDataset;
  const openFile = () => Source.open(dataset.name, resource.name);
  const openMetadata = () => Source.open(dataset.name, resource.name + ".toml");
</script>

<Card class="mb-3">
  <CardHeader>
    <CardTitle>
      {resource.name}
      <Button class="float-end btn-link" on:click={openFile}>Open file</Button>
    </CardTitle>
  </CardHeader>
  <CardBody>
    <Accordion>
      <AccordionItem>
        <CardSubtitle slot="header">
          Resource Metadata
          {#if !resource.metadata}
            <span class="badge bg-danger bg-opacity-25 text-black ms-3">
              Metadata is missing
            </span>
          {/if}
        </CardSubtitle>
        <CardText>
          {#if resource.metadata}
            <NiceMetadata metadata={resource.metadata} />
          {:else}
            <Alert color="danger">
              <h4>Metadata is missing</h4>
              <p>
                The selected folder doesn’t have metadata. You will not upload a
                dataset without it. This app can create metadata, but you should
                change some data there in the next stage.
              </p>

              <Button
                color="primary"
                on:click={() =>
                  Source.saveResource(dataset.name, resource.name)}
              >
                Create metadata automatically
              </Button>
            </Alert>
          {/if}
        </CardText>

        {#if resource.metadata}
          <CardFooter>
            <Button color="primary" outline on:click={openMetadata}>
              Open metadata in editor
            </Button>
          </CardFooter>
        {/if}
      </AccordionItem>
    </Accordion>
  </CardBody>
</Card>
