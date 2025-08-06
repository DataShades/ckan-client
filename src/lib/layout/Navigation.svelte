<script lang="ts">
  import { Button, Container, Icon, Spinner } from "sveltestrap";
  import { link, navigate } from "svelte-routing";
  import { Project, Flakes, Source, Queue, Submission, User } from "../../services";
  import { Manual } from "../component";

  export let step = 1;

  let available = 1;
  $: {
    if (!$Project) {
      available = 1;
    } else if (!$Source || !$Source.path || !$Source.metadata) {
      available = 2;
    } else {
      available = 3;
    }
  }
  $: everythingIsValid = $Source.datasets.every((d) =>
    $Flakes.ready.includes(d.name)
  );
  const checkCompletion = (notReady, ...datasets) =>

    !notReady.length && datasets.every((dataset) => {
      return dataset.resources.every((r) => {
        const details = $Flakes.uploads[`${dataset.name}/${r.name}`];
        return details && details.data.completed;
      })
    }
      
    );
  $: isCompleted = checkCompletion($Flakes.notReady, ...$Source.datasets);

  function onFinish() {
    navigate("/project");
    Queue.unfinalize();
    Source.change("");
    Submission.refresh();
    Project.reset($User);
  }
</script>
<div class="w-100 progress-stepper">
  <Container>
    <ul class="d-flex justify-content-evenly list-inline mb-0">
      {#if $Queue.processing}
        <li class:active={true} class="text-black">
          <span class="step-circle">
            <Spinner size="sm" />
          </span>
          Uploading...
        </li>
      {:else if  $Queue.finalized }
        <li class:active={true} class="text-black d-flex flex-column gap-3 text-center">
          <div>
            <span class="step-circle">
              <Icon name="check" />
            </span>
            Upload completed
          </div>
          <div class="d-flex align-items-center gap-2">
            To start uploading again, press
            <Button
              on:click={onFinish}
              color="primary"
              class="btn-sm"
              >
              finish
            </Button>

          </div>
        </li>
      {:else}
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
      {/if}
    </ul>
  </Container>
</div>

<Manual section={step} />

<div class="mt-4 main-page-content">
  <slot />
</div>


{#if !$Queue.processing && !$Queue.finalized}
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
          <a use:link class="btn btn-outline-primary" href="/project">
            Previous
          </a>
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
          <a use:link class="btn btn-primary" href="/source">Previous</a>
        </div>
        <div>
          <Button
            color="primary"
            disabled={!$Source.datasets.length ||
              !everythingIsValid ||
              $Queue.processing}
            on:click={async () => {
              navigate("/uploading");
              Queue.fullUpload(true);
            }}
          >
            Upload all datasets
          </Button>
        </div>
      {/if}
    </Container>
  </div>
{/if}
