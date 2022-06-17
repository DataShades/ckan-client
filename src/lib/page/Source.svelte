<script lang="ts">
  import { Alert, Button, FormGroup, Input, InputGroup } from "sveltestrap";
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
  const browse = async () => {
    let p = await Source.browse();
    if (p !== null) {
      setTimeout( () => {
        path = String(p);
        checkPath();
      }, 500);
    }
  };
</script>

<FormGroup class="m-5">
  <InputGroup>
    <Button on:click={browse}>Browse</Button>
    <Input placeholder="Path to the submission source" bind:value={path} />
  </InputGroup>
  <Button on:click={checkPath} class="mt-2">Synchronize</Button>
  {#if $Source.path}
    <Button on:click={() => Source.open()} class="mt-2">Open directory</Button>
  {/if}
  {#if $Source.metadata}
    <Button on:click={() => Source.open("metadata.json")} class="mt-2"
      >Open metadata</Button
    >
  {/if}
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
    <h3>Master metadata</h3>
    <pre>{JSON.stringify($Source.metadata, null, 2)}</pre>
  </div>
{/if}
