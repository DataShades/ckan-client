<svelte:options immutable={true}/>

<script lang="ts">
    import { Button, Container, Icon } from "sveltestrap";
    import { link } from "svelte-routing";
    import { Project, Source } from "../../services";
    import { Manual } from "../component";

    export let step = 1;

    let available = 1;
    $: {
        if (!$Project) {
            available = 1;
        } else if (!$Source || $Source.path || !$Source.metadata) {
            available = 2;
        } else {
            available = 3;
        }
    }
</script>

<div class="w-100 progress-stepper">
    <Container>
        <ul class="d-flex justify-content-between list-inline mb-0">
            <li class:active={step === 1} class:available={available >= 1}>
                <a use:link class="nav-link p-0" href="/project">
                    <span class="step-circle">
                        {#if available > 1}
                            <Icon name="check" />
                        {:else}
                            1
                        {/if}
                    </span>
                    Select a project
                </a>
            </li>
            <li class:active={step === 2} class:available={available >= 2}>
                <a
                    use:link
                    class="nav-link p-0"
                    class:disabled={available < 2}
                    href="/source"
                >
                    <span class="step-circle">
                        {#if available > 2}
                            <Icon name="check" />
                        {:else}
                            2
                        {/if}
                    </span>
                    Select a source and Add metadata
                </a>
            </li>
            <li class:active={step === 3} class:available={available >= 3}>
                <a
                    use:link
                    class="nav-link p-0"
                    class:disabled={available < 3}
                    href="/datasets"
                >
                    <span class="step-circle"> 3 </span>
                    Validate datasets and upload
                </a>
            </li>
        </ul>
    </Container>
</div>

<Manual section={step} />

<div class="mt-4 main-page-content"
    >
    <slot />
</div>

<div class="progress-footer fixed-bottom">
    <Container class="justify-content-between align-items-center d-flex">
        {#if step === 1}
            <div />
            <div>
                <a
                    use:link
                    class="btn btn-primary"
                    class:disabled={available <= step}
                    href="/source">Next</a
                >
            </div>
        {:else if step === 2}
            <div>
                <a
                    use:link
                    class="btn btn-outline-primary"
                    href="/project">Previous</a
                >
            </div>
            <div>
                <a
                    use:link
                    class="btn btn-primary"
                    class:disabled={available <= step}
                    href="/datasets">Next</a
                >
            </div>
        {:else}
            <div>
                <a
                    use:link
                    class="btn btn-primary"
                    href="/source">Previous</a
                >
            </div>
            <div>
                <Button color="primary">Complete</Button>
            </div>
        {/if}
    </Container>
</div>
