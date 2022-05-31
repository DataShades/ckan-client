<script lang="ts">
  import {
    Collapse,
    Navbar,
    NavbarToggler,
    NavbarBrand,
    Nav,
    NavItem,
    NavLink,
    Dropdown,
    DropdownToggle,
    DropdownMenu,
    DropdownItem
  } from 'sveltestrap';
  import {User} from "../../services"

  let isOpen = false;

  function handleUpdate(event) {
    isOpen = event.detail.isOpen;
  }
</script>

<Navbar color="light" light expand="md">
  <NavbarBrand href="#">
    {#if $User}
      Hello, {$User.fullname}!
    {:else}
      Hello
    {/if}
  </NavbarBrand>
  <NavbarToggler on:click={() => (isOpen = !isOpen)} />
    <Collapse {isOpen} navbar expand="md" on:update={handleUpdate}>
      <Nav class="ms-auto" navbar>
        {#if $User}
          <NavItem>
            <NavLink on:click={User.logout}>Change user</NavLink>
          </NavItem>
        <!--   <Dropdown nav inNavbar> -->
        <!--     <DropdownToggle nav caret>Options</DropdownToggle> -->
        <!--     <DropdownMenu end> -->
        <!--       <DropdownItem>Option 1</DropdownItem> -->
        <!--       <DropdownItem divider /> -->
        <!--       <DropdownItem>Reset</DropdownItem> -->
        <!--     </DropdownMenu> -->
        <!--   </Dropdown> -->
        {/if}
        </Nav>
    </Collapse>
</Navbar>
