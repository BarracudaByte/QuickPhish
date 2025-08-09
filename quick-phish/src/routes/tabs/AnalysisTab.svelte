<script>
// @ts-nocheck
    import FileAnalysis from "../analysis/FileAnalysis.svelte";
    import FileList from "../analysis/FileList.svelte";
    import FileUpload from "../analysis/FileUpload.svelte";
    import { open } from '@tauri-apps/plugin-dialog';
    import { invoke } from '@tauri-apps/api/core';
    import DOMPurify from 'dompurify';

    let eml = $state({});

    async function openFile() {
        const file = await open({
            multiple: false,
            directory: false,
        });
        console.log(`Open file: ${file}`);
        invoke('load_eml', { uri: file }).then((data) => {
            data.body = DOMPurify.sanitize(data.body);
            eml = data;
        });
    }


</script>


<div class="p-2">
    {#if eml.subject || eml.body }
        <h1>Analysis Results</h1>

        <h2>Summary</h2>
        <div class="border rounded-lg border-slate-200 dark:border-slate-700 p-2 my-2 font-mono font-light text-sm">
            {eml.summary}
            <!--<p>The email was sent from {eml.from} to {eml.to} with subject '{eml.subject}'</p>-->
        </div>

        <h2>Headers</h2>
        <div class="border rounded-lg border-slate-200 dark:border-slate-700 relative overflow-y-auto max-h-96">
            <table class="table-fixed w-full border-collapse">
                <thead class="sticky top-0 z-10 bg-slate-200 dark:bg-slate-700">
                    <tr class="">
                        <th class="px-5 py-3 text-xs font-medium text-left uppercase">Header</th>
                        <th class="px-5 py-3 text-xs font-medium text-left uppercase">Value</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-slate-200 dark:divide-slate-700 overflow-y-auto">
                    {#if eml.headers }
                        {#each Object.entries(eml.headers) as [k, v]}
                            <tr class="">
                                <td class="px-4 py-3 text-sm font-medium max-w-sm whitespace-nowrap overflow-x-auto">{k}</td>
                                <td class="px-4 py-3 text-sm whitespace-nowrap max-w-[50%] overflow-x-auto">{v}</td>
                            </tr>
                        {/each}
                    {/if}
                </tbody>
            </table>
        </div>
        <h2>Email &amp; Indicators</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 ">
            <div class="col-span-1 xl:col-span-2 border rounded-lg border-slate-200 dark:border-slate-700 p-2 my-2 font-light text-sm max-h-80 overflow-auto">
                {@html eml.body}
            </div>
            <div class="border rounded-lg border-slate-200 dark:border-slate-700 p-2 my-2 max-h-80 overflow-auto">
                <h3>URLs</h3>
                <ul>
                    {#each eml.indicators.urls as url}
                    <li>{url}</li>
                    {/each}
                </ul>
                <h3>Emails</h3>
                <ul>
                    {#each eml.indicators.emails as email}
                    <li>{email}</li>
                    {/each}
                </ul>
            </div>
        </div>
        <!--<div>{eml.body}</div>-->
    {:else if eml.error}
        <p>Error: {eml.error}</p>
    {/if}

    <button onclick={openFile} class="text-blue-500 hover:text-blue-600 cursor-pointer">Open .eml</button>


    <!--<p>EML: {JSON.stringify(eml)}</p>-->
</div>
