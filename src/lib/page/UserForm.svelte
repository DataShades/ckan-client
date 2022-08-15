<script lang="ts">
  import how1 from "../../assets/how1.png";
  import how2 from "../../assets/how2.png";
  import how3 from "../../assets/how3.png";
  import logo1 from "../../assets/logo1.png";
  import logo2 from "../../assets/logo2.png";

  import { createEventDispatcher } from "svelte";
  import {
    Image,
    Icon,
    Input,
    InputGroup,
    InputGroupText,
    Modal,
    ModalBody,
    ModalHeader,
    Spinner,
    Button,
    Container,
    Col,
    Row,
  } from "sveltestrap";

  export let pending = false;
  export let url = "";
  export let token = "";

  const dispatch = createEventDispatcher();
  const fdpUrl = "https://flooddata.ses.nsw.gov.au";

  $: disabled = !url || !token;

  let isModalOpen = false;

  const login = () => dispatch("login", { url, token });
  const toggleModal = () => (isModalOpen = !isModalOpen);
</script>

<div class="mt-4">
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
    <Input
      placeholder="Paste API token here"
      bind:value={token}
      type="password"
    />
  </InputGroup>

  <p class="mt-2">
    See an <span class="btn-link pointable" on:click={toggleModal}
      >instruction</span
    > to get an API token.
  </p>

  <Button on:click={login} {disabled} color="primary">
    Login
    {#if pending}
      <Spinner type="grow" size="sm" />
    {/if}
  </Button>
</div>

<Container
  class="justify-content-between align-items-center fixed-bottom mb-5 d-flex"
>
  <Image src={logo1} class="w-auto h-100" />
  <Image src={logo2} class="w-auto h-100" />
</Container>
<Modal isOpen={isModalOpen} toggle={toggleModal} size="lg">
  <ModalHeader toggle={toggleModal}>
    How to get an FDP authentification token
  </ModalHeader>
  <ModalBody>
    <p>Follow next steps to get API token for enter to the app.</p>
    <p>
      <Image fluid class="w-100" src={how1} />
    </p>
    <p>
      1. <strong>Log in</strong> to the
      <a class="color-orange" href={fdpUrl} target="_blank">Flood Data Portal</a
      >
      and <strong>go to your account dashboard</strong>.
    </p>
    <p>
      <Image fluid class="w-100" src={how2} />
    </p>
    <p>2. Go to the <strong>API Tokens</strong> tab.</p>
    <p>3. <strong>Enter any name</strong> of the token as you wish.</p>
    <p>
      4. Press <strong>Create API Token</strong> button.
    </p>
    <p>
      <Image fluid class="w-100" src={how3} />
    </p>
    <p>
      5. Press <strong>Copy</strong> button and <strong>paste</strong> to the specific
      field into Log in screen of the app.
    </p>
    <Button color="primary" on:click={toggleModal}>Back to Login</Button>
    <a class="btn btn-outline-primary" href={fdpUrl} target="_blank"
      >Go to Flood Data Portal</a
    >
  </ModalBody>
</Modal>
