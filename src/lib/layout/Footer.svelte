<script lang="ts">
  import { Toast, ToastBody, ToastHeader } from "sveltestrap";
  import {Toaster} from "../../services";

</script>


<div class="toaster m-2">
  {#each $Toaster as toast (toast.id)}
    <Toast class="m-1" autohide on:close={() => Toaster.pop(toast.id)}>
      {#if toast.header}
        <ToastHeader icon={toast.color}>
          {toast.header}
        </ToastHeader>
      {/if}
      <ToastBody>
        {toast.body}
      </ToastBody>

    </Toast>
  {/each}
</div>
