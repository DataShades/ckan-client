<svelte:options immutable/>
<script lang="ts">
  import type { TDataset } from "src/types";

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
  import { Submission, Source, Flakes } from "../../services";
  import { Resource } from ".";

  export let dataset: TDataset;
  export let validated: any | null;
</script>

<Card class="mb-3">
  <CardHeader>
    <CardTitle>{dataset.name}</CardTitle>
  </CardHeader>
  <CardBody>
    <Accordion>
      {#if dataset.metadata}
        <AccordionItem active>
          <CardSubtitle slot="header">Metadata</CardSubtitle>
          <CardText>
            <pre>{JSON.stringify(dataset.metadata, null, 2)}</pre>
          </CardText>
        </AccordionItem>
      {:else}
        <AccordionItem active>
          <CardSubtitle slot="header">Metadata is missing</CardSubtitle>
          <CardText>
            This dataset does not have metadata file. Click
            <Button on:click={() => Source.saveDataset(dataset.name)}>
              this button
            </Button>
            in order to create it.
          </CardText>
        </AccordionItem>
      {/if}
      <AccordionItem>
        <CardSubtitle slot="header">Resources</CardSubtitle>
        <CardText>
          {#if dataset.resources.length}
            {#each dataset.resources as resource}
              <Resource {resource} {dataset}/>
            {/each}
          {:else}
            This dataset has no resources yet.
          {/if}
        </CardText>
      </AccordionItem>

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
              Preview of the final dataset
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
    <Button on:click={() => Source.open(dataset.name)}>Open directory</Button>
    {#if dataset.metadata}
      <Button on:click={() => Source.open(dataset.name + ".json")}>
        Open metadata
      </Button>
      <Button on:click={() => Submission.validateDataset(dataset.name)}>
        Validate metadata
      </Button>
      <Button disabled on:click={() => {}}>Save</Button>
    {/if}
  </CardFooter>
</Card>
