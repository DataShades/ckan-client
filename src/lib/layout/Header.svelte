<script lang="ts">
  import {
    Collapse,
    Navbar,
    NavbarToggler,
    NavbarBrand,
    Nav,
    NavItem,
    NavLink,
    Container,
    Button,
  } from "sveltestrap";
  import { User } from "../../services";

  let isOpen = false;

  function handleUpdate(event: CustomEvent) {
    isOpen = event.detail.isOpen;
  }
  function logout() {
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
      <Button color="light" class="float-end" outline={true} on:click={logout}>Log out</Button>
    {/if}
  </Container>
</Navbar>
