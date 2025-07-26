<script>
// @ts-nocheck
    import { invoke } from "@tauri-apps/api/core";
    import { nav } from "./shared.svelte";
    import Sidebar from "./Sidebar.svelte";
    import FileUpload from "./analysis/FileUpload.svelte";
    import AnalysisTab from "./tabs/AnalysisTab.svelte";
    import TemplatesTab from "./tabs/TemplatesTab.svelte";

    let name = $state("");
    let greetMsg = $state("");

    async function greet(event) {
      event.preventDefault();
      // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
      greetMsg = await invoke("greet", { name });
    }
</script>

<div class="flex bg-zinc-50 dark:bg-zinc-900 text-black dark:text-white">
  <Sidebar />
  <div class="w-full">
    {#if nav.tab == 0 }
      <AnalysisTab />
    {:else if nav.tab == 1 }
      <TemplatesTab />
    {:else if nav.tab == 2 }
      <p>Actual Content. Currently selected tab: {nav.tab}</p>
    {/if}
  </div>
</div>