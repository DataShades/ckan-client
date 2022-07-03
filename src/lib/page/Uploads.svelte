<script lang="ts">
  import { Card, CardHeader, ListGroup, ListGroupItem } from "sveltestrap";

  import { Source, Queue, Flakes, Submission } from "../../services";
  import { Upload } from "../component";

  const upload = (e: CustomEvent) => {
    const { dataset, resource } = e.detail;
    Queue.add(dataset, resource);
    Queue.process();
  };

  const pause = (e: CustomEvent) => {
    const { dataset, resource } = e.detail;
    Queue.drop(dataset, resource);
  };
</script>

<ul>Upload queue:
{#each [...$Queue.items] as [key, _value]}
  <li>
    {key}
  </li>
{:else}
  Empty
{/each}
</ul>

<div class="m-5">
  {#each $Source.datasets.filter((d) => d.metadata && d.resources.length) as dataset}
    <Card class="m2">
      <CardHeader>{dataset.name}</CardHeader>
      <ListGroup>
        {#each dataset.resources as resource}
          <ListGroupItem>
            <Upload
              {dataset}
              {resource}
              details={$Flakes.uploads[`${dataset.name}/${resource.name}`]}
              queued={$Queue.items.has(`${dataset.name}/${resource.name}`)}
              on:upload={upload}
              on:pause={pause}
            />
          </ListGroupItem>
        {/each}
      </ListGroup>
    </Card>
  {/each}
</div>
