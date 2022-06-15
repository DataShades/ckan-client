<script lang="ts">
  import { Card, CardHeader, ListGroup, ListGroupItem } from "sveltestrap";

  import { Source, Flakes } from "../../services";
  import { Upload } from "../component";
</script>

<div class="m-5">
  {#each $Source.datasets.filter((d) => d.metadata) as dataset}
    <Card class="m2">
      <CardHeader>{dataset.name}</CardHeader>
      {#if dataset.resources.length}
        <ListGroup>
          {#each dataset.resources as resource}
            <ListGroupItem>
              <Upload {dataset} {resource} details={$Flakes.uploads[`${dataset.name}/${resource.name}`]} />
            </ListGroupItem>
          {/each}
        </ListGroup>
      {/if}
    </Card>
  {/each}
</div>
