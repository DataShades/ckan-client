<script lang="ts">
  import { createEventDispatcher } from "svelte";
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
  import { Source, Flakes, Queue } from "../../services";
  import { Resource } from ".";

  const dispatch = createEventDispatcher();

  export let dataset: TDataset;
  export let validated: any | null;
  export let pending: boolean;

  function humanizeSize(bytes) {
    if (bytes == 0) { return "0.00 B"; }
    var e = Math.floor(Math.log(bytes) / Math.log(1024));
    return (bytes/Math.pow(1024, e)).toFixed(2)+' '+' KMGTP'.charAt(e)+'B';
}
  $: size = humanizeSize(dataset.resources.reduce((total, r) => total + (r.size || 0), 0));

  let uploading: number = 0;
  Queue.subscribe(q => {
    uploading = dataset.resources.filter(resource => Queue.contains(dataset, resource)).length
  })
</script>

<Card class="mb-3">
  <CardHeader>
    <CardTitle>{dataset.name}</CardTitle>
  </CardHeader>
  <CardBody>
    <Accordion>
      {#if dataset.metadata}
        <AccordionItem>
          <CardSubtitle slot="header">Metadata</CardSubtitle>
          <CardText>
            <pre>{JSON.stringify(dataset.metadata, null, 2)}</pre>
          </CardText>
        </AccordionItem>
      {:else}
        <AccordionItem>
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
      {#if dataset.resources.length}
        <AccordionItem>
          <CardSubtitle slot="header">Resources</CardSubtitle>
          <CardText>
            {#each dataset.resources as resource}
              <Resource
                {resource}
                {dataset}
                validated={$Flakes.resources[
                  `${dataset.name}/${resource.name}`
                ]}
              />
            {/each}
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
      <Button on:click={() => Source.open(dataset.name + ".toml")}>
        Open metadata
      </Button>
      <Button
        disabled={pending}
        on:click={() => dispatch("validate", { dataset })}
      >
        Validate
      </Button>

      <Button disabled={uploading >= dataset.resources.length || !$Flakes.ready.includes(dataset.name)} on:click={() => dispatch("upload", { dataset })}>
        Upload({size})
      </Button>
    {/if}
  </CardFooter>
</Card>
