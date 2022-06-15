<script lang="ts">
  import type { TDataset, TResource } from "src/types";

  import { Button, ButtonGroup, Icon, Progress } from "sveltestrap";

  import { Submission } from "../../services";

  export let details: any;
  export let resource: TResource;
  export let dataset: TDataset;

  let pending = false;

  const register = async () => {
    pending = true;
    console.log('before')
    Submission.registerUpload(dataset.name, resource.name);
    console.log('after')
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

{resource.name}

<ButtonGroup disabled size="sm" class="float-end">
  <Button
    disabled={!resource.metadata || details || pending}
    on:click={register}
    title="Prepare for upload"
  >
    <Icon name="cloud-check" />
    prepare
  </Button>

  <Button
    disabled={!details || pending || details.data.completed}
    on:click={upload}
    title="Start/resume upload"
  >
    <Icon name="cloud-upload" />
    start/resume
  </Button>
  <Button disabled title="Pause upload">
    <Icon name="pause-circle" />
    pause
  </Button>
</ButtonGroup>

{#if details}
  <hr/>
  <Progress
    striped
    animated={pending}
    color={details.data.completed ? "success" : "info"}
    value={((details.data.bytes_uploaded + 1) / (details.data.size + 1)) * 100}
    />
{/if}
