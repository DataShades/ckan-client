<script lang="ts">
  import { Alert, Button, FormGroup, Input, Label } from "sveltestrap";
  import { Source, Toaster } from "../../services";

  let path = $Source.path;
  let error = "";

  const checkPath = async () => {
    error = "";
    try {
      await Source.change(path);
    } catch (e) {
      if (path) {
        error = e;
      }
    }
  };

  const save = async () => {
    try {
      await Source.saveRoot();
    } catch (e) {
      Toaster.error(e, "Error");
    }
  };
</script>

<FormGroup floating class="m-5">
  <Input placeholder="Path to the submission source" bind:value={path} />
  <Label slot="label">Path to the submission source</Label>
  <Button on:click={checkPath}>Synchronize</Button>
</FormGroup>

{#if error}
  <Alert color="danger" class="m-5">
    <h4>Cannot use the path as a source</h4>
    {error}
  </Alert>
{/if}

{#if $Source.path && !$Source.metadata}
  <Alert color="warning" class="m-5">
    <h4>Metadata file is not available.</h4>
    You can create
    <code>metadata.json</code>
    manually or click
    <Button size="sm" on:click={save}>this button</Button>
    in order to create it automatically.
    <br />
    If metadata file exists, make sure it contains a valid JSON object -
    <code>{"{}"}</code>

  </Alert>
{:else if $Source.path && $Source.metadata}
  <div class="m-5">
    <h3>Metadata</h3>
    <pre>{JSON.stringify($Source.metadata, null, 2)}</pre>
  </div>
{/if}
