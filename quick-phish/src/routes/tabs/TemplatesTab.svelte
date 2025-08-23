<script>
// @ts-nocheck
    import { invoke } from '@tauri-apps/api/core';
    import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
    import { context } from "../shared.svelte";
    import JsonViewer from '../lib/JsonViewer.svelte';

    let summary = $state("");
    let loading = $state(false);
    let updated = $state(false);

    invoke('get_summary_template').then((data) => {summary = data.template; console.log(data);});
    
    async function updateSummaryTemplate() {
        loading = true;
        invoke('update_summary_template', { summary: summary }).then((ok) => { 
            console.log(`Update Template: ${ok}`);
            loading = false;
            if (ok) {
                updated = true;
            }
        });
    }

    function inputChanged() {
        updated = false;
        console.log("Input has changed...");
    }



</script>

<div class="m-2 h-dvh">
    <h1>Templates</h1>
    <p>Update the default templates to automatically generate case notes the way you need them. All templates support the Jinja syntax.</p>
    <h2 class="mt-4">Email Summary</h2>
    <textarea type="text" bind:value={summary} oninput={inputChanged} placeholder="Summary Template" class="flex h-auto min-h-[80px] w-full px-3 py-2 text-sm border rounded-md border-slate-200 dark:border-slate-700 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-slate-300 focus:dark:ring-slate-700 disabled:cursor-not-allowed disabled:opacity-50"></textarea>
    <div class="flex flex-row-reverse my-2">
        {#if loading}
            <div class="animate-spin inline-block size-6 border-3 border-current border-t-transparent text-blue-600 rounded-full dark:text-blue-500" role="status" aria-label="loading">
                <span class="sr-only">Loading...</span>
            </div>
        {:else if updated }
            <p class="text-green-500">&#10003;</p>
        {:else}
            <button onclick={updateSummaryTemplate} class="rounded bg-blue-500 hover:bg-blue-600 text-white px-2 py-1 cursor-pointer">Update</button>
        {/if}
    </div>
    <h2 class="mt-4">Data Structure</h2>
    {#if context.eml.subject || context.eml.body }
        <div class="flex min-h-[80px] max-h-[240px] w-full overflow-auto px-3 py-2 text-sm border rounded-md border-slate-200 dark:border-slate-700 ">
            <JsonViewer data={context}/>
        </div>
    {:else}
        <p>Open an eml in the analysis tab to see its structure here.</p>
    {/if}

    
</div>
