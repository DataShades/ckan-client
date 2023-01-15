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
    Container,
    FormGroup,
    Input,
    InputGroup,
  } from "sveltestrap";
  import { Source, Toaster } from "../../services";
import { NiceMetadata } from "../component";

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
      setTimeout(() => {
        path = String(p);
        checkPath();
      }, 500);
    }
  };
</script>

<Container>
  <h2 class="page-title">Select a source</h2>
  <p>Select a folder containing dataset folders</p>
  <FormGroup>
    <InputGroup>
      <Button
        on:click={browse}
        class="btn-select-source"
        color="secondary"
        outline
      >
        Browse
      </Button>
      <Input
        placeholder="Select a folder that consists dataset directory"
        bind:value={path}
      />
    </InputGroup>
    {#if $Source.path}
      <Button
        on:click={() => Source.open()}
        color="primary"
        outline
        class="mt-3">Open folder</Button
      >
    {/if}
  </FormGroup>

  {#if error}
    <Alert color="danger">
      <h4>Cannot use the path as a source</h4>
      {error}
    </Alert>
  {/if}

  {#if $Source.path && !$Source.metadata}
    <Alert color="danger">
      <h4>Metadata is missing</h4>
      <p>
        The selected folder doesn’t have metadata. You will not upload a dataset
        without it. This app can create metadata, but you should change some
        data there in the next stage.
      </p>
      <Button color="primary" on:click={save}
        >Create metadata automatically</Button
      >
    </Alert>
  {:else if $Source.path && $Source.metadata}
    <Card>
      <CardHeader>
        <CardTitle>Master metadata</CardTitle>
      </CardHeader>
      <CardBody>
        <CardText>
          <NiceMetadata metadata={$Source.metadata}/>
        </CardText>
      </CardBody>
      <CardFooter>
        <Button
          color="primary"
          outline
          on:click={() => Source.open("metadata.toml")}
        >
          Open metadata in editor
        </Button>
      </CardFooter>
    </Card>
  {/if}
</Container>
