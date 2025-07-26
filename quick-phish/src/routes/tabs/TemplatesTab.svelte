<script>
// @ts-nocheck
    import { invoke } from '@tauri-apps/api/core';

    let summary = $state("");

    invoke('get_summary_template').then((data) => {summary = data.template; console.log(data);});
    
    async function updateSummaryTemplate() {
        invoke('update_summary_template', { summary: summary }).then((ok) => console.log(`Update Template: ${ok}`));
    }
</script>

<div>
    <h1>Templates</h1>

    <h2>Email Summary</h2>
    <textarea type="text" bind:value={summary} placeholder="Summary Template" class="flex h-auto min-h-[80px] px-3 py-2 text-sm border rounded-md border-slate-200 dark:border-slate-700 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-slate-300 focus:dark:ring-slate-700 disabled:cursor-not-allowed disabled:opacity-50"></textarea>
    <div class="flex flex-row-reverse">
        <button onclick={updateSummaryTemplate}>Update</button>
    </div>


    
</div>
