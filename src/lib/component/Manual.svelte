<script lang="ts">
    import { Button, Icon, Offcanvas } from "sveltestrap";
    export let open = false;

    const toggle = () => (open = !open);
</script>

<div class="manual-wrapper">
    <Button color="light" class="manual-toggle" on:click={toggle}>
        <Icon name="question-square-fill" />
    </Button>

    <Offcanvas style="width: 90%" container="inline" isOpen={open} {toggle} header="Start" placement="start">
        Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua.
    </Offcanvas>
</div>
