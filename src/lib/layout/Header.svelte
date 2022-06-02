<script lang="ts">

  import {
    Collapse,
    Navbar,
    NavbarToggler,
    NavbarBrand,
    Nav,
    NavItem,
    NavLink,
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

<Navbar color="dark" dark expand="md">
  <NavbarBrand href="#">
    {#if $User}
      Hello, {$User.display_name}!
    {:else}
      Hello
    {/if}
  </NavbarBrand>
  <NavbarToggler on:click={() => (isOpen = !isOpen)} />
    <Collapse {isOpen} navbar expand="md" on:update={handleUpdate}>
      <Nav class="ms-auto" navbar>
        {#if $User}
          <NavItem>
            <NavLink on:click={logout}>Change user</NavLink>
          </NavItem>
        {/if}
      </Nav>
    </Collapse>
</Navbar>
