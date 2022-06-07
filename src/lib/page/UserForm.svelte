<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import {
    Button,
    Icon,
    Input,
    InputGroup,
    InputGroupText,
    Spinner,
  } from "sveltestrap";

  const dispatch = createEventDispatcher();

  export let disabled = false;
  export let url = "";
  export let token = "";

  const login = () => dispatch("login", { url, token });
</script>

<div class="m-5">
  <InputGroup>
    <InputGroupText>
      <Icon name="globe" />
    </InputGroupText>
    <Input type="text" placeholder="Portal URL" bind:value={url} />
  </InputGroup>

  <br />

  <InputGroup>
    <InputGroupText>
      <Icon name="key" />
    </InputGroupText>
    <Input placeholder="API Token" bind:value={token} type="password" />
  </InputGroup>

  <br />

  <Button on:click={login} {disabled} color="success">
    Login
    {#if disabled}
      <Spinner type="grow" size="sm" />
    {/if}
  </Button>
</div>
