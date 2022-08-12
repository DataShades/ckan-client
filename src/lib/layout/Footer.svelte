<script lang="ts">
  import { Toast, ToastBody, ToastHeader } from "sveltestrap";
  import { Toaster } from "../../services";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
</script>

<div class="toaster m-2">
  {#each $Toaster as toast (toast.id)}
    <div transition:fly={{ y: 200, duration: 200 }} animate:flip={{duration: 200}}>
      <Toast  class="m-1 toast-{toast.color}"  >
          <ToastHeader toggle={() => Toaster.pop(toast.id)}>
            {toast.header || "Attention"}
          </ToastHeader>
        <ToastBody>
          {toast.body}
        </ToastBody>
      </Toast>
    </div>
  {/each}
</div>
