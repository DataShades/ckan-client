<script lang="ts">
  import type { TDataset } from "src/types";

  import {
    Accordion,
    AccordionItem,
    Alert,
    Button,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    CardText,
    CardTitle,
    Container,
  } from "sveltestrap";

  import { Source, Flakes, Submission } from "../../services";
  import { Dataset, NiceMetadata } from "../component";
  import DatasetErrors from "../component/DatasetErrors.svelte";

  let pending: string | null = null;

  const scrollToDataset = (name: string) => {
    const el = document.getElementById(btoa(name));
    if (!el) {
      console.error("Cannot locate element for name %s", name);
      return;
    }
    el.scrollIntoView({ behavior: "smooth" });
  };
  const validate = async (dataset: TDataset) => {
    pending = dataset.name;
    await Submission.validateDataset(dataset.name);
    for (let resource of dataset.resources) {
      await Submission.validateResource(dataset.name, resource.name);
    }
    pending = null;
  };
  $: totalErrors = Object.keys($Flakes.datasets)
    .filter((k) => $Flakes.notReady.includes(k))
    .map((k) => Object.keys($Flakes.datasets[k].extras.errors).length)
    .reduce((t, n) => t + n, 0);
</script>
<Container class="validate-page">
  <h2 class="page-title">Validate datasets and upload</h2>
  <p>After successful validation, upload your datasets to the portal</p>
  <p>
    <Button
      color="primary"
      outline
      on:click={() => Submission.validateEverything()}
    >
      Validate all
    </Button>
  </p>
  <Card>
    <CardHeader>
      <CardTitle
        style="display:flex;"
        class="justify-content-between align-items-center"
      >
        <span>
          Validation errors
          {#if totalErrors}
            <span class="badge bg-danger bg-opacity-25 text-black ">
              {totalErrors} error{totalErrors > 1 ? "s" : ""}
            </span>
          {/if}
        </span>
        <Button
          color="primary"
          outline
          class="float-end"
          on:click={() => Source.open("metadata.csv")}
        >
          Open metadata in editor
        </Button>
      </CardTitle>
    </CardHeader>
    <CardBody>
      <CardText>
        {#if $Flakes.notReady.length}
          {#each $Source.datasets as dataset}
            <DatasetErrors
              {dataset}
              validated={$Flakes.datasets[dataset.name]}
              on:scrollToDataset={() => scrollToDataset(dataset.name)}
            />
          {/each}
        {:else}
          <Alert color="success">All files are valid.</Alert>
        {/if}
      </CardText>
    </CardBody>
  </Card>
  <div class="mt-4">
    <Accordion>
      <AccordionItem class="master-metadata-accordion" header="Master Metadata">
        <Card>
          <CardBody>
            <CardText>
              <NiceMetadata metadata={$Source.metadata} />
            </CardText>
          </CardBody>
          <CardFooter>
            <Button
              color="primary"
              outline
              on:click={() => Source.open("metadata.csv")}
            >
              Open metadata in editor
            </Button>
          </CardFooter>
        </Card>
      </AccordionItem>
    </Accordion>
  </div>
  <div class="mt-4">
    {#each $Source.datasets as dataset}
      <Dataset
        {dataset}
        validated={$Flakes.datasets[dataset.name]}
        pending={pending === dataset.name}
        on:validate={(e) => validate(e.detail.dataset)}
      />
    {/each}
  </div>
</Container>
