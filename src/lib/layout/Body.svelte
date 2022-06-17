<script lang="ts">
  import { Button, ButtonGroup, Container, Nav, NavItem } from "sveltestrap";
  import { Router, Link, Route, navigate, link } from "svelte-routing";
  import { UserForm, Home, Project, Datasets, Uploads, Source } from "../page";
  import {
    Portal,
    User,
    Project as SProject,
    Source as SSource,
    Queue,
    Flakes,
    Submission,
  } from "../../services";

  let loginDisabled = false;
  $: project = $SProject;
  $: source = $SSource;
  const login = async (e: CustomEvent) => {
    loginDisabled = true;

    await Portal.persist(e.detail);
    const user = await User.resolve(e.detail);

    loginDisabled = false;
    if (user) {
      navigate("/");
    }
  };

  $: projectSelected = !!project;
  $: sourceReady = !!(source && source.path && source.metadata);
  $: hasAnyDatasets = $SSource.datasets.length;
  $: everythingIsValid = $SSource.datasets.every((d) =>
    $Flakes.ready.includes(d.name)
  );
</script>

<Container fluid class="flex-grow-1 mt-2">
  <Router>
    {#if $User}
      <Nav tabs>
        <NavItem>
          <Link class="nav-link" to="/">Hint</Link>
        </NavItem>
        <NavItem>
          <Link class="nav-link" to="/project">Project</Link>
        </NavItem>
        <NavItem>
          <a
            use:link
            class:nav-link={true}
            class:disabled={!projectSelected}
            href="/source"
            title="Requires a Project"
          >
            Source
          </a>
        </NavItem>
        <NavItem>
          <a
            use:link
            class:nav-link={true}
            class:disabled={!projectSelected || !sourceReady}
            href="/datasets"
            title="Requires a Source"
          >
            Datasets
          </a>
        </NavItem>
        <NavItem>
          <a
            use:link
            class:nav-link={true}
            class:disabled={!projectSelected || !sourceReady}
            href="/uploads"
            title="Requires a Source"
          >
            Uploads
          </a>
        </NavItem>
        <NavItem class="ms-auto">
          {#if hasAnyDatasets}
            <ButtonGroup>
              <Button
                color="success"
                class="nav-link"
                on:click={() => Submission.validateEverything()}
              >
                Validate all datasets
              </Button>
              {#if everythingIsValid}
                {#if $Queue.processing}
                  <Button
                    color="success"
                    class="nav-link"
                    on:click={() => Queue.clear()}
                  >
                    Pause all uploads
                  </Button>
                {:else}
                  <Button
                    color="success"
                    class="nav-link"
                    on:click={async () => Queue.fullUpload(true)}
                  >
                    Upload everything and complete submission
                  </Button>
                {/if}
              {/if}
            </ButtonGroup>
          {/if}
        </NavItem>
      </Nav>

      <Route path="/"><Home {project} {source} /></Route>
      <Route path="project"><Project chosen={project} user={$User} /></Route>
      <Route path="source"><Source /></Route>
      <Route path="datasets"><Datasets /></Route>
      <Route path="uploads"><Uploads /></Route>
    {:else}
      <UserForm
        url={$Portal.url}
        token={$Portal.token}
        on:login={login}
        disabled={loginDisabled}
      />
    {/if}
  </Router>
</Container>
