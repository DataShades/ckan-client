<script lang="ts">
  import { ListGroup, ListGroupItem } from "sveltestrap";
  import type {TProject, TSource} from "../../types";

  export let project: TProject;
  export let source: TSource;
  let hasProject = !!project
  let hasSource = !!source.path;
  let readyForCompletion = false;
</script>

<ListGroup numbered color="success" class="m-5">
  <ListGroupItem active={!hasProject}>Select a project</ListGroupItem>
  <ListGroupItem active={hasProject && !hasSource} disabled={!hasProject}>
    Select source
  </ListGroupItem>
  <ListGroupItem active={hasProject && hasSource} disabled={!hasProject || !hasSource}>Validate metadata</ListGroupItem>
  <ListGroupItem active={hasProject && hasSource} disabled={!hasProject || !hasSource}>Create datasets</ListGroupItem>
  <ListGroupItem active={hasProject && hasSource} disabled={!hasProject || !hasSource}>Upload files</ListGroupItem>
  <ListGroupItem disabled={!readyForCompletion}>Complete submission</ListGroupItem>
</ListGroup>
