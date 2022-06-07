<script lang="ts">
  import { onMount } from "svelte";

  import {
    Alert,
    Button,
    ButtonGroup,
    FormGroup,
    Icon,
    Input,
    Label,
    ListGroup,
    ListGroupItem,
    Spinner,
  } from "sveltestrap";
  import { Project } from "../../services";
  import type { TProject, TUser } from "../../types";

  export let user: TUser;
  export let chosen: TProject;
  let name = "";
  let queueId: any;
  let pending = true;
  let projects: TProject[] = [];

  onMount(search);

  function queue(e: InputEvent) {
    clearTimeout(queueId);
    queueId = setTimeout(search, 500);
  }

  async function search() {
    pending = true;
    projects = await Project.filter(name);
    pending = false;
  }
</script>

<FormGroup floating class="m-5">
  <Input
    placeholder="Project name or title"
    on:input={queue}
    bind:value={name}
  />
  <Label slot="label">
    Project name or title
    {#if pending}<Spinner size="sm" />{/if}
  </Label>
</FormGroup>

{#if projects.length}
  <ListGroup>
    {#each projects as proj (proj.id)}
      <ListGroupItem active={chosen && proj.id === chosen.id}>
        {proj.title}({proj.name})
        <ButtonGroup size="sm" class="float-end">
          {#if chosen && proj.id === chosen.id}
            <Button on:click={() => Project.reset(user)}>
              <Icon name="eraser" />
            </Button>
          {:else}
            <Button on:click={() => Project.choose(proj, user)}>
              <Icon name="bag-check" />
            </Button>
          {/if}
        </ButtonGroup>
      </ListGroupItem>
    {/each}
  </ListGroup>
{:else if !pending}
  <Alert color="secondary">
    <h4 class="alert-heading text-capitalize">Nothing found</h4>
    Try searching by the project's title or name(URL).
  </Alert>
{/if}
