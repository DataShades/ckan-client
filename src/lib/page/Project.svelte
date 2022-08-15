<script lang="ts">
  import { onMount } from "svelte";

  import {
    Alert,
    Button,
    ButtonGroup,
    Container,
    FormCheck,
    FormGroup,
    Input,
  } from "sveltestrap";
  import { Project } from "../../services";
  import type { TProject, TUser } from "../../types";

  export let user: TUser;
  export let chosen: TProject;
  let name = "";
  let pending = true;
  let projects: TProject[] = [];

  onMount(search);

  async function search() {
    pending = true;
    projects = await Project.filter(name);
    pending = false;
  }
</script>

  <Container>
    <h2 class="page-title">Select a project</h2>
    <FormGroup class="d-flex">
      <Input placeholder="Enter flood project name" bind:value={name} />
      <Button
        on:click={search}
        disabled={pending}
        color="primary"
        class="ms-2"
        outline>Search</Button
      >
    </FormGroup>

    {#if projects.length}
      <ul class="projects-list p-0">
        {#each projects as proj (proj.id)}
          <li class="project-item pointable" on:click={() => Project.choose(proj, user)}>
            {proj.title}
            <ButtonGroup size="sm" class="float-end">
              <FormCheck checked={chosen && chosen.id == proj.id}/>
            </ButtonGroup>
          </li>
        {/each}
      </ul>
    {:else if !pending}
      <Alert color="secondary">
        <h4 class="alert-heading text-capitalize">Nothing found</h4>
        Try searching by the project's title or name(URL).
      </Alert>
    {/if}
  </Container>
