<script lang="ts">
  import { ListGroup, ListGroupItem } from "sveltestrap";
  import type {TProject} from "../../types";

  export let project: TProject;
  export let source: string;
</script>

<ListGroup numbered color="success" class="m-5">
  <ListGroupItem active={!project}>Select a project</ListGroupItem>
  <ListGroupItem disabled={!project} active={!!project && !source}>
    Select source
  </ListGroupItem>
  <ListGroupItem disabled={!project || !source}>Validate metadata</ListGroupItem>
  <ListGroupItem disabled={!project || !source}>Create datasets</ListGroupItem>
  <ListGroupItem disabled={!project || !source}>Upload files</ListGroupItem>
  <ListGroupItem disabled={!project || !source}>Complete submission</ListGroupItem>
</ListGroup>
