<script lang="ts">
  import { Alert, Button, FormGroup, Input, Label } from "sveltestrap";
  import { Source } from "../../services";
  import type { TProject } from "../../types";

  export let project: TProject;
  let source = $Source;
  let content: any;
  let error = "";

  const checkPath = async () => {
    error = ""
    try {
      content = await Source.change(source);
    } catch (e) {
      error = e;
    }

  }
</script>

<FormGroup floating class="m-5">
  <Input placeholder="Path to the submission source" bind:value={source} />
  <Label slot="label">Path to the submission source</Label>
  <Button on:click={checkPath} class="mt-2">
    Check
  </Button>
</FormGroup>

{#if error}
  <Alert color="danger" class="m-5">
    <h4>Cannot use the path as a source</h4>
    {error}
  </Alert>
{/if}

{content}
