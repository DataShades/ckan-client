<script lang="ts">
  import { humanizeSize } from "../../utils";
  import { createEventDispatcher } from "svelte";

  import {
    Accordion,
    AccordionItem,
    Alert,
    Button,
    Icon,
    Offcanvas,
    Progress,
    Spinner,
  } from "sveltestrap";

  import { Source, Queue, Flakes } from "../../services";
  import { Upload } from "../component";
  import type { TDataset } from "../../types";

  const dispatch = createEventDispatcher();
  const upload = (e: CustomEvent) => {
    const { dataset, resource } = e.detail;
    Queue.add(dataset, resource);
    Queue.process();
  };

  const pause = (dataset, resource) => {
    Queue.drop(dataset, resource);
  };
  const pauseDataset = (dataset) => {
    dataset.resources.forEach((resource) => pause(dataset, resource));
  };
  let isOpen = false;
  const isCompleted = (...datasets) =>
    datasets.every((dataset) =>
      dataset.resources.every((r) => {
        const details = $Flakes.uploads[`${dataset.name}/${r.name}`];
        return details && details.data.completed;
      })
    );
  const toggle = () => (isOpen = !isOpen);
  $: totalSize = humanizeSize(
    $Source.datasets.reduce(
      (total: number, d: TDataset) =>
        total + d.resources.reduce((total, r) => total + (r.size || 0), 0),
      0
    )
  );

  const progress = (...datasets) => {
    let total = { size: 0, uploaded: 0 };
    for (let dataset of datasets) {
      Object.entries($Flakes.uploads)
        .filter(([k, v]) => k.indexOf(dataset.name + "/") === 0)
        .reduce((total, [_, next]) => {
          total.size += next.data.size;
          total.uploaded += next.data.bytes_uploaded;
          return total;
        }, total);
    }
    // Add one byte to both parts avoiding ZeroDivision
    return (((total.uploaded + 1) / (total.size + 1)) * 100).toFixed(0);
  };
  $: everythingIsValid = $Source.datasets.every((d) =>
    $Flakes.ready.includes(d.name)
  );
</script>

<div class="offcanvas-wrapper">
  <Button
    color="light"
    class="uploading-offcanvas-toggle offcanvas-toggle offcanvas-toggle-right"
    on:click={toggle}
  >
    {#if $Queue.finalized}
      <Icon name="cloud-upload-fill" />
    {:else}
      <Spinner size="sm" type="grow" color="primary" />

      <span style="font-size: 1rem"> Uploading... </span>
    {/if}
  </Button>
  <Offcanvas
    style="width: 90%"
    container="inline"
    class="uploads"
    {isOpen}
    {toggle}
    placement="end"
    header="Uploads"
  >
    <div class="global-upload-actions">
      <Button
        color={isCompleted(...$Source.datasets) ? "link" : "primary"}
        disabled={!$Source.datasets.length ||
          !everythingIsValid ||
          $Queue.processing}
        on:click={async () => Queue.fullUpload(true)}
      >
        <!-- on:click={() => Queue.clear()} -->
        <Icon name="cloud-upload-fill" />
        {#if isCompleted(...$Source.datasets)}
          Uploading completed
        {:else}
          Upload all
        {/if}
        ({totalSize})
      </Button>
      <Button
        color="primary"
        outline
        class="ms-2"
        disabled={!$Queue.processing}
        on:click={async () => Queue.clear()}
      >
        <Icon name="pause-circle-fill" />
        Pause all
      </Button>
      <br />
      <div class:opacity-75={!$Queue.processing}>
        <Progress
          value={progress(...$Source.datasets)}
          class="mt-3"
          color="primary"
        >
          {progress(...$Source.datasets)}%
        </Progress>
      </div>
    </div>
    <div class="upload-listing">
      {#if !everythingIsValid}
        <Alert color="danger">Validate first to start uploading.</Alert>
      {/if}

      <div>
        {#each $Source.datasets.filter((d) => d.metadata && d.resources.length) as dataset}
          <Accordion>
            <AccordionItem class="mt-3">
              <div slot="header" class="dataset-progress-item">
                {dataset.name}
                <br />
                <Button
                  color={isCompleted(dataset) ? "link" : "primary"}
                  outline={!isCompleted(dataset)}
                  disabled={!dataset.metadata ||
                    !$Flakes.ready.includes(dataset.name)}
                  on:click={(e) => {
                    e.stopPropagation();
                    dispatch("upload", { dataset });
                  }}
                >
                  <Icon name="cloud-upload-fill" />

                  {#if isCompleted(dataset)}
                    Uploading completed
                  {:else}
                    Upload
                  {/if}
                  ({humanizeSize(
                    dataset.resources.reduce(
                      (total, r) => total + (r.size || 0),
                      0
                    )
                  )})
                </Button>
                <Button
                  color="link"
                  on:click={(e) => {
                    e.stopPropagation();
                    pauseDataset(dataset);
                  }}
                >
                  <Icon name="pause-circle-fill" />
                </Button>
                <div class:opacity-75={isCompleted(dataset)}>
                  <Progress color="primary" value={progress(dataset)}>
                    {progress(dataset)}%
                  </Progress>
                </div>
              </div>

              <ul class="list-unstyled">
                {#each dataset.resources as resource}
                  <li class="resource-progress-item mt-3">
                    <Upload
                      {dataset}
                      {resource}
                      details={$Flakes.uploads[
                        `${dataset.name}/${resource.name}`
                      ]}
                      queued={$Queue.items.has(
                        `${dataset.name}/${resource.name}`
                      )}
                      on:upload={upload}
                      on:pause={(e) =>
                        pause(e.detail.dataset, e.detail.resource)}
                    />
                  </li>
                {/each}
              </ul>
            </AccordionItem>
          </Accordion>
        {/each}
      </div>
    </div>
  </Offcanvas>
</div>
