<script lang="ts">
  import { initialiseStorage } from "$lib/api/app";

  let storagePath = $state<string | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);

  async function handleInitialiseStorage() {
    loading = true;
    error = null;

    try {
      storagePath = await initialiseStorage();
    } catch (err: unknown) {
      error = String(err);
    } finally {
      loading = false;
    }
  }
</script>

<section
  class="rounded-xl border border-stash-border
         bg-stash-sidebar p-5"
>
  <div class="mb-4">
    <h3 class="text-base font-semibold">
      Local storage
    </h3>

    <p class="mt-1 text-sm text-stash-muted">
      Your bookmark library is stored on your device.
    </p>
  </div>

  {#if storagePath}
    <div class="rounded-lg bg-stash-surface p-3">
      <p class="mb-1 text-xs text-stash-muted">
        Storage directory
      </p>

      <p class="break-all font-mono text-sm">
        {storagePath}
      </p>
    </div>
  {:else}
    <p class="mb-4 text-sm text-stash-muted">
      Storage has not been initialised in this session.
    </p>
  {/if}

  {#if error}
    <p
      role="alert"
      class="mt-3 text-sm text-red-400"
    >
      {error}
    </p>
  {/if}

  <button
    type="button"
    onclick={handleInitialiseStorage}
    disabled={loading}
    class="mt-4 rounded-lg bg-stash-primary
           px-4 py-2.5 text-sm font-semibold
           text-white transition-opacity
           hover:opacity-90
           disabled:cursor-not-allowed
           disabled:opacity-50"
  >
    {loading ? "Initialising..." : "Initialise storage"}
  </button>
</section>