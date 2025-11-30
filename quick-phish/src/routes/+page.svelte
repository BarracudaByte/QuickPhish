<script>
// @ts-nocheck
    import { invoke } from "@tauri-apps/api/core";
    import { nav } from "./shared.svelte";
    import Sidebar from "./Sidebar.svelte";
    import FileUpload from "./analysis/FileUpload.svelte";
    import AnalysisTab from "./tabs/AnalysisTab.svelte";
    import TemplatesTab from "./tabs/TemplatesTab.svelte";
    import ListTab from "./tabs/ListTab.svelte";
    import SettingsTab from "./tabs/SettingsTab.svelte";

    let name = $state("");
    let greetMsg = $state("");

    async function greet(event) {
      event.preventDefault();
      // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
      greetMsg = await invoke("greet", { name });
    }
</script>

<Sidebar />
<div class="flex bg-zinc-50 {nav.expanded ? 'ml-48' : 'ml-12'} dark:bg-zinc-900 text-black dark:text-white overflow-auto">
  <div class="w-full h-full xl:px-8">
    {#if nav.tab == 0 }
      <AnalysisTab />
    {:else if nav.tab == 1 }
      <TemplatesTab />
    {:else if nav.tab == 2 }
      <ListTab />
    {:else if nav.tab == 3 }
      <SettingsTab />
    {/if}
  </div>
</div>