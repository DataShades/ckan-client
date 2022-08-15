<script lang="ts">
  import type { TDataset } from "src/types";
  import { humanizeSize } from "../../utils";

  import {
    Accordion,
    AccordionItem,
    Button,
    Card,
    CardBody,
    CardFooter,
    CardText,
    Container,
  } from "sveltestrap";

  import { Source, Flakes, Submission, Queue } from "../../services";
  import { Dataset, NiceMetadata, Uploads } from "../component";

  let pending: string | null = null;

  const validate = async (dataset: TDataset) => {
    pending = dataset.name;
    await Submission.validateDataset(dataset.name);
    for (let resource of dataset.resources) {
      await Submission.validateResource(dataset.name, resource.name);
    }
    pending = null;
  };
  const upload = async (dataset: TDataset) => {
    dataset.resources.forEach((r) => Queue.add(dataset, r));
    Queue.process();
  };
  $: totalSize = humanizeSize(
    $Source.datasets.reduce(
      (total: number, d: TDataset) =>
        total +
        d.resources.reduce((total, r) => total + (r.size || 0), 0),
      0
    )
  );

  $: everythingIsValid = $Source.datasets.every((d) =>
    $Flakes.ready.includes(d.name)
  );
</script>
<Uploads/>
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
    <Button
      color="primary"
      class="ms-2"
      disabled={!$Source.datasets.length ||
        !everythingIsValid ||
        $Queue.processing}
      on:click={async () => Queue.fullUpload(false)}
    >
      <!-- on:click={() => Queue.clear()} -->
      Upload all ({totalSize})
    </Button>
  </p>
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
            on:click={() => Source.open("metadata.toml")}
          >
            Open metadata in editor
          </Button>
        </CardFooter>
      </Card>
    </AccordionItem>
  </Accordion>
  <div class="mt-4">
    {#each $Source.datasets as dataset}
      <Dataset
        {dataset}
        validated={$Flakes.datasets[dataset.name]}
        pending={pending === dataset.name}
        on:validate={(e) => validate(e.detail.dataset)}
        on:upload={(e) => upload(e.detail.dataset)}
      />
    {/each}
  </div>
</Container>
