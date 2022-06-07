<script lang="ts">
  import { Toast, ToastBody, ToastHeader } from "sveltestrap";
  import { Toaster } from "../../services";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
</script>

<div class="toaster m-2">
  {#each $Toaster as toast (toast.id)}
    <div transition:fly={{ y: 200, duration: 200 }} animate:flip={{duration: 200}}>
      <Toast  class="m-1" autohide on:close={() => Toaster.pop(toast.id)}>
        {#if toast.header}
          <ToastHeader icon={toast.color}>
            {toast.header}
          </ToastHeader>
        {/if}
        <ToastBody>
          {toast.body}
        </ToastBody>
      </Toast>
    </div>
  {/each}
</div>
