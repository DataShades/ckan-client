<script lang="ts">
  import { Progress } from "sveltestrap";

  export let total = 0;
  export let empty = 0;
  export let ready = 0;

  $: invalid = total - ready - empty;
</script>

<div class="fixed-bottom">
  <Progress multi style="border-radius: 0;">
    <Progress
      max={total}
      bar
      color="warning"
      value={empty}
      title="Without metadata: {empty}"
    />
    <Progress
      max={total}
      bar
      color="danger"
      value={invalid}
      title="Not validated: {invalid}"
    />
    <Progress
      max={total}
      bar
      color="info"
      value={ready}
      title="Ready for upload: {ready}"
    />
  </Progress>
</div>
