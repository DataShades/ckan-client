<script lang="ts">
  import { Container, Nav, NavItem } from "sveltestrap";
  import { Router, Link, Route, navigate } from "svelte-routing";
  import { UserForm, Home, Project, Datasets, Uploads, Source } from "../page";
  import {
    Portal,
    User,
    Project as SProject,
    Source as SSource,
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
</script>

<Container fluid class="flex-grow-1 mt-2">
  <Router>
    {#if $User}
      <Nav tabs>
        <NavItem>
          <Link class="nav-link" to="/">Hint</Link>
        </NavItem>
        <NavItem>
          <Link class="nav-link" to="project">Project</Link>
        </NavItem>
        {#if project}
          <NavItem>
            <Link class="nav-link" to="source">Source</Link>
          </NavItem>
          {#if source.path}
            <NavItem>
              <Link class="nav-link" to="datasets">Datasets</Link>
            </NavItem>
            <NavItem>
              <Link class="nav-link" to="uploads">Uploads</Link>
            </NavItem>
          {/if}
        {/if}
      </Nav>

      <Route path="/"><Home {project} {source} /></Route>
      <Route path="project"><Project chosen={project} user={$User} /></Route>
      <Route path="source"><Source/></Route>
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
