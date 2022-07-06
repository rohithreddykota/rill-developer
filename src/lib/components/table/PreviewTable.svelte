<script lang="ts">
  /**
   * PreviewTable.svelte
   * Use this component to drop into the application.
   * Its goal it so utilize all of the other container components
   * and provide the interactions needed to do things with the table.
   */
  import { onMount } from "svelte";
  import { Table, TableRow, TableCell } from "$lib/components/table/";
  import PreviewTableHeader from "./PreviewTableHeader.svelte";
  import TableHeader from "./TableHeader.svelte";
  import {
    MODEL_PREVIEW_PAGE_LENGTH,
    MODEL_PREVIEW_MAX_LENGTH,
  } from "$common/constants";

  interface ColumnName {
    name: string;
    type: string;
  }

  export let columnNames: ColumnName[];
  export let rows: any[];
  export let fetchMore: () => Promise<void>;

  const MAX_COLUMN_WIDTH = "250px";

  let selectedColumns = [];
  let activeIndex;

  function columnIsPinned(name, selectedCols) {
    return selectedCols.map((column) => column.name).includes(name);
  }

  function togglePin(name, type, selectedCols) {
    // if column is already pinned, remove.
    if (columnIsPinned(name, selectedCols)) {
      selectedColumns = [
        ...selectedCols.filter((column) => column.name !== name),
      ];
    } else {
      selectedColumns = [...selectedCols, { name, type }];
    }
  }

  // infinite scrolling
  const lastRowObserver = new IntersectionObserver(
    async (entries) => {
      const lastRow = entries[0];
      if (!lastRow.isIntersecting) return;
      console.log("trigger row is visible");
      await fetchMore();
      lastRowObserver.unobserve(lastRow.target);
      if (
        rows.length % MODEL_PREVIEW_PAGE_LENGTH === 0 &&
        rows.length < MODEL_PREVIEW_MAX_LENGTH
        // rows.length < model.cardinality // TODO: switch cardinality to totalRows
      ) {
        lastRowObserver.observe(document.querySelector(`tr:last-child > th`));
      }
    },
    { rootMargin: "0px" }
  );

  // TODO: figure out when to attach this observer.
  // Notes:
  // - component needs to be mounted. however `onMount` only triggers once, so it's not sufficient.
  // - race condition between the persistent model query and the derived model error
  // - model.id and other attributes are somehow being continuously updated by the state service (see hack below)
  let tableElement;
  function attachLastRowObserver(): void {
    console.log("attempting to attach last row observer");
    if (!tableElement) return;
    // TODO: && rows.length < model.cardinality
    if (rows.length % MODEL_PREVIEW_PAGE_LENGTH === 0) {
      lastRowObserver.observe(document.querySelector(`tr:last-child > th`));
    }
  }

  onMount(() => {
    attachLastRowObserver();
  });

  // hack: currently, reactive statements dependent on `model.ATTRIBUTE` continuously update, so they're not much help
  // adding another reactive layer, modelATTRIBUTE, ensures that downstream reactive statements will only trigger when the value truly changes
  // $: rowsHack = rows; // not quite right
  // reactive statement to attach the observer when the rows change
  // $: rowsHack, attachLastRowObserver();
</script>

<div class="flex relative">
  <Table bind:this={tableElement}>
    <!-- headers -->
    <TableRow>
      <TableHeader position="top-left">#</TableHeader>
      {#each columnNames as { name, type } (name)}
        {@const thisColumnIsPinned = columnIsPinned(name, selectedColumns)}
        <PreviewTableHeader
          {name}
          {type}
          pinned={thisColumnIsPinned}
          on:pin={() => {
            togglePin(name, type, selectedColumns);
          }}
          maxWidth={MAX_COLUMN_WIDTH}
        />
      {/each}
    </TableRow>
    <!-- values -->
    {#each rows as row, index}
      <TableRow hovered={activeIndex === index && activeIndex !== undefined}>
        <TableHeader position="left">{index + 1}</TableHeader>
        {#each columnNames as { name, type } (index + name)}
          <TableCell
            {type}
            value={row[name]}
            isNull={row[name] === null}
            maxWidth={MAX_COLUMN_WIDTH}
          />
        {/each}
      </TableRow>
    {/each}
  </Table>

  {#if selectedColumns.length}
    <div
      class="sticky right-0 z-20 bg-white border border-l-4 border-y-0 border-r-0 border-gray-300"
    >
      <Table>
        <TableRow>
          {#each selectedColumns as { name, type } (name)}
            {@const thisColumnIsPinned = columnIsPinned(name, selectedColumns)}
            <PreviewTableHeader
              {name}
              {type}
              maxWidth={MAX_COLUMN_WIDTH}
              pinned={thisColumnIsPinned}
              on:pin={() => {
                togglePin(name, type, selectedColumns);
              }}
            />
          {/each}
        </TableRow>
        {#each rows as row, index}
          <TableRow
            hovered={activeIndex === index && activeIndex !== undefined}
          >
            {#each selectedColumns as { name, type }}
              <TableCell
                {type}
                {index}
                isNull={row[name] === null}
                value={row[name]}
                maxWidth={MAX_COLUMN_WIDTH}
              />
            {/each}
          </TableRow>
        {/each}
      </Table>
    </div>
  {/if}
</div>
