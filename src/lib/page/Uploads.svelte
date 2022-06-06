<script lang="ts">
  import {
    Alert,
    Button,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    CardTitle,
    FormGroup,
    Input,
    InputGroup,
  } from "sveltestrap";

  import { Source, Flakes } from "../../services";
  import { Resource } from "../component";
  let names = {};
</script>

<div class="m-5">
  {#if !$Source.datasets.length}
    <Alert color="primary">The source contains no datasets.</Alert>
  {:else}
    {#each $Source.datasets.filter((d) => d.metadata) as dataset}
      <Card class="mb-3">
        <CardHeader>
          <CardTitle>{dataset.name}</CardTitle>
        </CardHeader>
        <CardBody>
          {#if dataset.resources.length}
            {#each dataset.resources as resource}
              <Resource {resource} {dataset} details={$Flakes.uploads[`${dataset.name}/${resource.name}`]}/>
            {/each}
          {:else}
            This dataset has no resources yet.
          {/if}
        </CardBody>
        <CardFooter>
          <FormGroup class="mt-1">
            <InputGroup>
              <Input
                placeholder="Resource name"
                bind:value={names[dataset.name]}
              />
              <Button
                on:click={() =>
                  Source.addResource(dataset.name, names[dataset.name])}
              >
                Create
              </Button>
            </InputGroup>
          </FormGroup>
        </CardFooter>
      </Card>
    {/each}
  {/if}
</div>
