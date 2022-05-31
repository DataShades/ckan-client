<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { Button, Icon, Input, InputGroup, InputGroupText, Spinner } from "sveltestrap";
  import {Theme} from "../../services";

  const dispatch = createEventDispatcher();

  export let disabled = false;
  export let url = "";
  export let token = "";

  const login = () => dispatch("login", {url, token})

</script>

<div class="m-5">
  <InputGroup>
    <InputGroupText>
      <div on:dblclick={Theme.next}>
        <Icon name="globe"/>
      </div>
    </InputGroupText>
    <Input type="text" placeholder="Portal URL" bind:value={url}/>
  </InputGroup>

  <br />

  <InputGroup>
    <InputGroupText>
      <Icon name="key"/>
    </InputGroupText>
    <Input placeholder="API Token" bind:value={token} type="password"/>
  </InputGroup>

  <br />

  <Button on:click={login} disabled={disabled} color="success">
    Login
    {#if disabled}
      <Spinner type="grow" size="sm"/>
    {/if}
  </Button>

</div>
