<script lang="ts">

  import "bootstrap-icons/font/bootstrap-icons.css"
  import "./styles/styles.css"

  import {Header, Body, Footer} from "./lib/layout"
  import {UserForm} from "./lib/page"
  import {Theme, Portal, User, Toaster} from "./services"
import Submission from "./lib/page/Submission.svelte"

  let loginDisabled = false;
  const login = async (e: CustomEvent) => {
    loginDisabled = true

    await Portal.persist(e.detail)
    const user = await User.resolve(e.detail);

    loginDisabled = false;
    if (user) {
      Toaster.info(`LoggedIn as ${user.fullname}`, "Login succeed")
    } else {
      Toaster.error("Wrong URL or token", "Login failed")
    }

  }


</script>

<main class="d-flex flex-column min-vh-100">

  <Header/>
  <Body>
    {#if $User}
      <Submission />
    {:else}
      <UserForm url={$Portal.url} token={$Portal.token} on:login={login} disabled={loginDisabled}/>
    {/if}
  </Body>
  <Footer/>
</main>

<svelte:head>
  <link rel="stylesheet" href="/src/assets/bootswatch/{$Theme}.min.css" type="text/css" media="screen" />
</svelte:head>
