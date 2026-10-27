<script lang="ts">
  type Collection = {
    name: string;
    icon: string;
  };

  let {
    activeCollection,
    onCollectionChange,
  }: {
    activeCollection: string;
    onCollectionChange: (name: string) => void;
  } = $props();

  const collections: Collection[] = [
    { name: "All bookmarks", icon: "▦" },
    { name: "Favourites", icon: "☆" },
  ];
</script>

<aside
  class="flex w-[245px] shrink-0 flex-col border-r
         border-stash-border bg-stash-sidebar
         px-4 py-7 max-[650px]:w-[170px]
         max-[650px]:px-2"
>
  <!-- Brand -->
  <div class="mb-12 flex items-center gap-3 px-3">
    <div
      class="grid size-[34px] place-items-center
             rounded-[10px] bg-stash-primary
             font-bold text-white"
    >
      S
    </div>

    <h1 class="m-0 text-2xl font-bold tracking-[-1px]">
      Stash<span class="text-stash-accent">.</span>
    </h1>
  </div>

  <!-- Library navigation -->
  <div>
    <p
      class="mb-3.5 px-3 text-[11px] font-bold
             tracking-[1.5px] text-stash-subtle"
    >
      LIBRARY
    </p>

    <nav
      aria-label="Bookmark collections"
      class="flex flex-col gap-1"
    >
      {#each collections as collection (collection.name)}
        <button
          type="button"
          onclick={() => onCollectionChange(collection.name)}
          class={[
            "flex w-full cursor-pointer items-center",
            "gap-3 rounded-lg px-3 py-3 text-left",
            "transition-colors duration-150",
            activeCollection === collection.name
              ? "bg-stash-selected text-stash-accent-light"
              : "text-stash-muted hover:bg-stash-hover",
          ]}
          aria-current={
            activeCollection === collection.name
              ? "page"
              : undefined
          }
        >
          <span
            class="w-5 text-center text-[19px]"
            aria-hidden="true"
          >
            {collection.icon}
          </span>

          <span class="text-sm font-medium">
            {collection.name}
          </span>
        </button>
      {/each}
    </nav>
  </div>

  <!-- Storage status -->
  <div
    class="mt-auto flex items-center gap-[9px]
           px-3 py-3.5 text-xs text-stash-muted"
  >
    <span
      class="size-[7px] rounded-full bg-stash-success"
      aria-hidden="true"
    ></span>

    Stored locally
  </div>
</aside>