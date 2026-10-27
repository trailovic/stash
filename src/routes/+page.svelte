<script lang="ts">
  import Sidebar from "$lib/components/Sidebar.svelte";
  import StorageSettings from "$lib/components/StorageSettings.svelte";

  let activeCollection = $state("All bookmarks");
</script>

<svelte:head>
  <title>Stash — Your personal library</title>
  <meta
    name="description"
    content="A local-first bookmark library."
  />
</svelte:head>

<div
  class="flex min-h-screen bg-stash-bg font-sans
         text-stash-text"
>

  <Sidebar
    {activeCollection}
    onCollectionChange={(name) => (activeCollection = name)}
  />

  <main
    class="min-w-0 flex-1 p-9
           max-[650px]:px-4 max-[650px]:py-[22px]"
  >
    <!-- Toolbar -->
    <header
      class="flex items-center justify-between gap-5
             max-[650px]:flex-col
             max-[650px]:items-start"
    >
      <div>
        <h2
          class="mb-1.5 text-[25px] font-semibold
                 tracking-tight"
        >
          {activeCollection}
        </h2>

        <p class="text-sm text-stash-muted">
          Your personal corner of the internet.
        </p>
      </div>

      <button
        type="button"
        disabled
        title="Coming soon"
        class="rounded-lg bg-stash-primary px-4 py-[11px]
               font-semibold text-white opacity-65
               disabled:cursor-not-allowed"
      >
        + Add bookmark
      </button>
    </header>

    <!-- Search -->
    <div class="mt-8">
      <input
        type="search"
        placeholder="Search your library..."
        aria-label="Search bookmarks"
        disabled
        class="w-full max-w-[480px] rounded-lg
               border border-[#303139]
               bg-stash-surface px-4 py-[13px]
               text-stash-text placeholder:text-stash-muted
               outline-none disabled:cursor-not-allowed
               disabled:opacity-70"
      />
    </div>

    <!-- Empty library -->
    <section
      class="mx-auto mt-[110px] flex max-w-[340px]
             flex-col items-center text-center"
    >
      <div
        class="mb-[22px] grid size-[70px]
               place-items-center rounded-[18px]
               bg-[#24212f] text-[42px]
               text-stash-accent"
        aria-hidden="true"
      >
        ◇
      </div>

      <h3
        class="mb-2.5 text-xl font-semibold"
      >
        Your library is empty
      </h3>

      <p
        class="text-sm leading-[1.7]
               text-stash-muted"
      >
        Your saved websites, articles and discoveries
        will appear here.
      </p>

      <span
        class="mt-3.5 text-xs text-stash-subtle"
      >
        Bookmark management is coming next.
      </span>
    </section>
    <div class="mx-auto mt-12 max-w-[480px]">
      <StorageSettings />
    </div>
  </main>
</div>