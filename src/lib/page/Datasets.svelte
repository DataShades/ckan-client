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
    FormGroup,
    Input,
    InputGroup,
    Label,
  } from "sveltestrap";

  import { Source } from "../../services";

  let newDatasetName = "";
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
              <Button on:click={() => Source.saveDataset(dataset.name)}
                >this button</Button
              >
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

  <FormGroup class="mt-5">
    <InputGroup>
      <Input placeholder="Dataset name" bind:value={newDatasetName} />
      <Button on:click={() => Source.addDataset(newDatasetName)}>
        Create
      </Button>
    </InputGroup>
  </FormGroup>
</div>
