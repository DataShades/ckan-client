<script lang="ts">
  import { Navbar, NavbarBrand, Container, Button } from "sveltestrap";
  import { User } from "../../services";

  async function logout() {
    User.logout();
  }
</script>

<Navbar color="primary" dark>
  <Container class="container">
    <NavbarBrand href="#">
      {#if $User}
        Hello, {$User.display_name}
      {:else}
        Login
      {/if}
    </NavbarBrand>
    {#if $User}
      <Button color="light" class="float-end" outline={true} on:click={logout}>
        Log out
      </Button>
    {/if}
  </Container>
</Navbar>
