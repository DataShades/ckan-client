<script lang="ts">
  import { Container } from "sveltestrap";
  import { Router, Route, navigate } from "svelte-routing";
  import { UserForm, Project, Datasets, Source } from "../page";

  import { Manual, Step } from "../../services";
  import { Portal, User, Project as SProject } from "../../services";
  import { Navigation } from "../layout";
  let loginDisabled = false;
  $: project = $SProject;
  const login = async (e: CustomEvent) => {
    loginDisabled = true;

    await Portal.persist(e.detail);
    const user = await User.resolve(e.detail);

    loginDisabled = false;
    if (user) {
      navigate("/project");
      console.log("Logged in", user);
      Manual.toggle(null);
    }
  };
</script>

<Container fluid class="flex-grow-1 p-0">
  <Router>
    {#if $User}
      <Navigation step={$Step}>
        <Route path="project"><Project chosen={project} user={$User} /></Route>
        <Route path="source"><Source /></Route>
        <Route path="datasets"><Datasets /></Route>
      </Navigation>
    {:else}
      <Container>
        <UserForm
          url={$Portal.url}
          token={$Portal.token}
          on:login={login}
          pending={loginDisabled}
        />
      </Container>
    {/if}
  </Router>
</Container>
