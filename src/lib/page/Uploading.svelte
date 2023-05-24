<script lang="ts">
  import type { TDataset } from "src/types";
  import { navigate } from "svelte-routing";
  import { Button, Container } from "sveltestrap";
  import { Flakes, Queue, Source, Submission } from "../../services";
  import { Uploads } from "../component";

  const upload = async (dataset: TDataset) => {
    dataset.resources.forEach((r) => Queue.add(dataset, r));
    Queue.process();
  };

  const checkCompletion = (...datasets) =>
    datasets.every((dataset) =>
      dataset.resources.every((r) => {
        const details = $Flakes.uploads[`${dataset.name}/${r.name}`];
        return details && details.data.completed;
      })
    );
  $: isCompleted = checkCompletion(...$Source.datasets);
</script>

<Uploads on:upload={(e) => upload(e.detail.dataset)} />

<Container>
  {#if isCompleted || $Queue.finalized}
    <h2>Upload completed</h2>
    <p>To start uploading again, press finish</p>
    <Button
      on:click={() => {
        navigate("/project");
        Queue.unfinalize();
        Source.change("");
        Submission.refresh()
      }}
      color="primary"
      class="mt-3"
    >
      Finish
    </Button>
  {:else}
    <h2>Uploading...</h2>
    <p>
      Please, wait until the upload will finish. You can manage uploads on the
      uploading sidebar.
    </p>
  {/if}
</Container>
