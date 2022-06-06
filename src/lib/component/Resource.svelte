<script lang="ts">
  import {
    Button,
    ButtonGroup,
    Icon,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    CardSubtitle,
    CardText,
    CardTitle,
    Progress,
  } from "sveltestrap";
  import type { TResource, TDataset } from "../../types";
  import { Source, Submission } from "../../services";

  export let resource: TResource;
  export let dataset: TDataset;
  export let details: any;

  let pending = false;
  const register = async () => {
    pending = true;
    await Submission.registerUpload(dataset.name, resource.name);
    pending = false;
  };

  const upload = async () => {
    pending = true;
    await Submission.progressUpload(
      dataset.name,
      resource.name,
      details.data.chunks_uploaded + 1
    );
    pending = false;
  };
</script>

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
        <Button
          on:click={() => Source.saveResource(dataset.name, resource.name)}
        >
          this button
        </Button>
        in order to create it.
      </CardText>
    {/if}
    {#if details}
      <Progress
        striped
        animated={pending}
        color={details.data.completed ? "success" : "info"}
        value={((details.data.bytes_uploaded + 1) / (details.data.size + 1)) *
          100}
      >
      </Progress>
    {/if}
  </CardBody>
  <CardFooter>
    <ButtonGroup disabled size="sm" class="float-end">
      <Button
        disabled={!resource.metadata || details || pending}
        on:click={register}
        title="Prepare for upload"
      >
        <Icon name="cloud-check" />
      </Button>

      <Button
        disabled={!details || pending || details.data.completed}
        on:click={upload}
        title="Start/resume upload"
      >
        <Icon name="cloud-upload" />
      </Button>
      <Button disabled title="Pause upload">
        <Icon name="pause-circle" />
      </Button>
    </ButtonGroup>
  </CardFooter>
</Card>
