<script lang="ts">
  import {
    Accordion,
    AccordionItem,
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
  export let validated: any | null;
  const openFile = () => Source.open(dataset.name, resource.name);
  const openMetadata = () => Source.open(dataset.name, resource.name + ".json");
</script>

<Card class="mb-3">
  <CardHeader>
    <CardTitle>{resource.name}</CardTitle>
  </CardHeader>
  <CardBody>
    <Accordion>
      {#if resource.metadata}
        <AccordionItem>
          <CardSubtitle slot="header">Metadata</CardSubtitle>
          <CardText>
            <pre>{JSON.stringify(resource.metadata, null, 2)}</pre>
          </CardText>
        </AccordionItem>
      {:else}
        <AccordionItem>
          <CardSubtitle slot="header">Metadata is missing</CardSubtitle>
          <CardText>
            This resource does not have metadata file. Click
            <Button
              on:click={() => Source.saveResource(dataset.name, resource.name)}
            >
              this button
            </Button>
            in order to create it.
          </CardText>
        </AccordionItem>
      {/if}
      {#if validated}
        {#if Object.keys(validated.extras.errors).length}
          <AccordionItem>
            <CardSubtitle slot="header">Errors</CardSubtitle>
            <CardText>
              <pre>{JSON.stringify(validated.extras.errors, null, 2)}</pre>
            </CardText>
          </AccordionItem>
        {:else}
          <AccordionItem>
            <CardSubtitle slot="header">
              Preview of the final resource
            </CardSubtitle>
            <CardText>
              <pre>{JSON.stringify(validated.data, null, 2)}</pre>
            </CardText>
          </AccordionItem>
        {/if}
      {/if}
    </Accordion>
  </CardBody>
  <CardFooter>
    <Button on:click={openFile}>Open file</Button>
    {#if resource.metadata}
      <Button on:click={openMetadata}>Open metadata</Button>
    {/if}
  </CardFooter>
</Card>
