<script lang="ts">
  import { slide } from "svelte/transition";
  import { cubicOut as easing } from "svelte/easing";

  import { dataModelerService } from "$lib/application-state-stores/application-store";
  import PreviewTable from "$lib/components/table/PreviewTable.svelte";
  import type { DerivedModelEntity } from "$common/data-modeler-state-service/entity-state-service/DerivedModelEntityService";
  import { MODEL_PREVIEW_PAGE_LENGTH } from "$common/constants";

  import {
    modelPreviewVisibilityTween,
    layout,
  } from "$lib/application-state-stores/layout-store";

  export let model: DerivedModelEntity;
  export let query: string;

  $: currentPreviewLength = model?.preview?.length;

  async function fetchMore(): Promise<void> {
    await dataModelerService.dispatch("updateModelPreview", [
      model.id,
      currentPreviewLength + MODEL_PREVIEW_PAGE_LENGTH,
    ]);
  }

  let previewTableContainer;
  function scrollToTop(): void {
    if (previewTableContainer) {
      const previewTable = document.querySelector("#preview-table-container");
      previewTable.scrollTop = 0;
    }
  }

  // reactive statement triggered when the query changes
  $: query, scrollToTop();
</script>

<div
  style:height="{(1 - $modelPreviewVisibilityTween) *
    $layout.modelPreviewHeight}px"
  class="p-6 "
>
  <div
    id="preview-table-container"
    class="rounded border border-gray-200 border-2 overflow-auto h-full"
    class:border={!!model?.error}
    class:border-gray-300={!!model?.error}
    bind:this={previewTableContainer}
  >
    {#if model?.error}
      <div
        transition:slide={{ duration: 200, easing }}
        class="error font-bold rounded-lg p-5 text-gray-700"
      >
        {model.error}
      </div>
    {:else if model?.preview && model?.profile}
      <PreviewTable
        rows={model.preview}
        columnNames={model.profile}
        {fetchMore}
      />
    {:else}
      <div class="grid items-center justify-center italic pt-3 text-gray-600">
        no columns selected
      </div>
    {/if}
  </div>
</div>
