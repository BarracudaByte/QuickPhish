<script>
// @ts-nocheck
    import FileAnalysis from "../analysis/FileAnalysis.svelte";
    import FileList from "../analysis/FileList.svelte";
    import FileUpload from "../analysis/FileUpload.svelte";
    import { open } from '@tauri-apps/plugin-dialog';
    import { invoke } from '@tauri-apps/api/core';

    let eml = $state({});

    async function openFile() {
        const file = await open({
            multiple: false,
            directory: false,
        });
        console.log(`Open file: ${file}`);
        invoke('load_eml', { uri: file }).then((data) => eml = data);
    }


</script>

<!--<FileUpload />-->
<!--<div class="flex">
    <FileList />
    <FileAnalysis />
</div>-->

<div class="p-2">
    {#if eml["Body"] }
        <h1 class="text-lg">Analysis Results</h1>
        <h2 class="text-lg">Summary</h2>
        <p>The email was sent from {eml["From"]} to {eml["To"]} with subject '{eml["Subject"]}'</p>
        <h2 class="text-lg">Headers</h2>
        <div class="border-x rounded-lg border-slate-200 dark:border-slate-700 relative overflow-y-auto max-h-96">
            <table class="table-fixed w-full border-collapse">
                <thead class="sticky top-0 z-10 bg-slate-200 dark:bg-slate-700">
                    <tr class="">
                        <th class="px-5 py-3 text-xs font-medium text-left uppercase">Header</th>
                        <th class="px-5 py-3 text-xs font-medium text-left uppercase">Value</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-slate-200 dark:divide-slate-700 overflow-y-auto">
                    {#each Object.entries(eml) as [k, v]}
                        {#if !["Body", "Subject", "To", "From"].includes(k) }
                            <tr class="">
                                <td class="px-4 py-3 text-sm font-medium max-w-sm whitespace-nowrap overflow-x-auto">{k}</td>
                                <td class="px-4 py-3 text-sm whitespace-nowrap max-w-[50%] overflow-x-auto">{v}</td>
                            </tr>
                        {/if}
                    {/each}
                </tbody>
            </table>
        </div>
        <div>
            Body: 
            {eml["Body"]}
        </div>
    {/if}
    <button on:click={openFile} class="text-blue-500 hover:text-blue-600 cursor-pointer">Open .eml</button>
</div>
