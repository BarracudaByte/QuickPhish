<script>
// @ts-nocheck
    import FileAnalysis from "../analysis/FileAnalysis.svelte";
    import FileList from "../analysis/FileList.svelte";
    import FileUpload from "../analysis/FileUpload.svelte";
    import { context } from "../shared.svelte";
    import { onMount, tick } from 'svelte';
    import { open } from '@tauri-apps/plugin-dialog';
    import { invoke } from '@tauri-apps/api/core';
    import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
    import { listen } from '@tauri-apps/api/event';
    import DOMPurify from 'dompurify';


    listen('open-file', () => {
        openFile();
    });

    //let eml = $state({});
    let loading = $state(false);
    let file = $state("");
    let copiedSummary = $state(false);
    let emailState = $state("Rendered");
    let showEmailViewOptions = $state(false);
    let emlContainer;

    async function openFile() {
        loading = true;
        file = await open({
            multiple: false,
            directory: false,
        });
        console.log(`Open file: ${file}`);
        invoke('load_eml', { uri: file }).then((data) => {
            var sanitized = DOMPurify.sanitize(data.body);
            data.body = updateLinks(sanitized);
            context.eml = data;
            loading = false;
        });
    }

    async function reloadAnalysis() {
        invoke('render_summary', { uri: file }).then((data) => {
            // TODO
        });
    }

    async function copySummary() {
        if (context.eml.summary) {
            await writeText(context.eml.summary);
            copiedSummary = true;
        }
    }

    function updateLinks(html) {
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, 'text/html');
        var allLinks = doc.querySelectorAll('a');
        console.log("Updating links...");
        allLinks.forEach(link => {
            link.setAttribute('data-href', link.href);
            link.setAttribute('id', link.href);
            link.removeAttribute('href');
            /*link.addEventListener('click', async (event) => {
                event.preventDefault();
                await writeText(this.id);
                console.log(`Copied "${link.id}" to clipboard!`);
            });*/
        });
        return doc.body.innerHTML;
    }

    function updateEmailView(view) {
        emailState = view;
        showEmailViewOptions = false;
    }

    $effect(() => {
        console.log(`Effect function! ${loading}`);
        if (emlContainer && context.eml && context.eml.body) {
            console.log("Inside the effect");
            emlContainer.innerHTML = context.eml.body;

            const links = emlContainer.querySelectorAll('a');
            links.forEach(link => {
                link.addEventListener('click', async (event) => {
                    event.preventDefault(); 
                    await writeText(link.id);
                    console.log(`Clicked on link to: ${link.id}`);
                });
            });
        }
    });
    
    /*onMount(async () => {
        console.log("Waiting for tick... ");
        // Wait for the DOM to be updated with the `finalHtml` value.
        await tick();
    //function doThis() {
        console.log("Do this... ");
        const links = emlDiv.querySelectorAll('a');

        links.forEach(link => {
            link.addEventListener('click', async (event) => {
                event.preventDefault();
                await writeText(this.id);
                console.log(`Copied "${link.id}" to clipboard!`);
            });
        });
    });
    //}*/
    
    


</script>


<div class="p-2 h-dvh">
    {#if context.eml.subject || context.eml.body }
        <div class="flex">
            <h1 class="grow">Analysis Results</h1><!--{ JSON.stringify(file) }-->
            <button onclick={openFile} class="mx-auto rounded bg-blue-500 hover:bg-blue-600 text-white px-2 py-1 cursor-pointer">Open New .eml</button>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 mt-2">
            <!-- Risk Score -->
            <div class="rounded bg-white dark:bg-zinc-800 shadow-sm px-3 pb-2">
                <div class="flex">
                    <h2 class="">Risk Score: {context.eml.riskScore.score}</h2>
                    <span class="relative flex size-3 mt-3 ms-2">
                        <span class="absolute inline-flex h-full w-full animate-ping rounded-full {context.eml.score == 0 ? 'bg-lime-400' : (context.eml.score == 1 ? 'bg-amber-400' : 'bg-rose-400')} opacity-75"></span>
                        <span class="relative inline-flex size-3 rounded-full {context.eml.score == 0 ? 'bg-lime-500' : (context.eml.score == 1 ? 'bg-amber-500' : 'bg-rose-500')}"></span>
                    </span>
                </div>
                <ul class="text-sm list-disc ms-4">
                    {#each context.eml.riskScore.reasons as reason }
                        <li>{reason}</li>
                    {/each }
                </ul>
            </div>

            <!-- Summary -->
            <div class="rounded bg-white dark:bg-zinc-800 shadow-sm px-3 pb-2 col-span-1 xl:col-span-2">
                <div class="flex">
                    <h2 class="grow">Summary</h2>
                    <!--<button aria-label="refresh" onclick={reloadAnalysis} class="p-2 text-black dark:text-white cursor-pointer hover:text-blue-600 mt-1">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5" class="size-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                        </svg>
                    </button>-->

                </div>
                <div class="border rounded-lg border-zinc-200 dark:border-zinc-700 p-2 my-2 font-mono font-light text-sm">
                    {context.eml.summary}
                </div>
                <div class="flex flex-row-reverse mb-2">
                    <button onclick={copySummary} class="border rounded flex gap-1 px-2 pt-1 mt-1 cursor-pointer text-xs hover:ring-1 {copiedSummary ? 'text-lime-500 border-lime-500  ring-emerald-500' : 'text-zinc-700 dark:text-zinc-300 border-zinc-500 ring-blue-500' }">
                        <span class="pt-1">Copy to Clipboard</span>
                        {#if copiedSummary}
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="0.8" stroke="currentColor" class="size-5 mb-1">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M11.35 3.836c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m8.9-4.414c.376.023.75.05 1.124.08 1.131.094 1.976 1.057 1.976 2.192V16.5A2.25 2.25 0 0 1 18 18.75h-2.25m-7.5-10.5H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V18.75m-7.5-10.5h6.375c.621 0 1.125.504 1.125 1.125v9.375m-8.25-3 1.5 1.5 3-3.75" />
                            </svg>
                        {:else}
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="0.8" stroke="currentColor" class="size-5 mb-1">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25ZM6.75 12h.008v.008H6.75V12Zm0 3h.008v.008H6.75V15Zm0 3h.008v.008H6.75V18Z" />
                            </svg>
                        {/if}
                    </button>
                </div>
            </div>
        </div>

        <div class="rounded bg-white dark:bg-zinc-800 shadow-sm px-3 py-2 mt-4">
            <h2>Headers</h2>
            <div class="rounded-lg relative overflow-y-auto max-h-80 pb-1">
                <table class="table-fixed w-full border-collapse  rounded-lg border-slate-200 dark:border-slate-700"><!--border-->
                    <thead class="sticky top-0 z-10 bg-zinc-200 dark:bg-zinc-700 rounded-t-lg">
                        <tr class="">
                            <th class="px-5 py-3 text-xs font-bold text-left uppercase">Key</th>
                            <th class="px-5 py-3 text-xs font-bold text-left uppercase">Value</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-zinc-200 dark:divide-zinc-700 overflow-y-auto rounded-b-lg">
                        {#if context.eml.headers }
                            {#each Object.entries(context.eml.headers) as [k, v]}
                                <tr class="hover:bg-zinc-50 hover:dark:bg-zinc-700/25"><!-- odd:bg-gray-100/50 odd:dark:bg-gray-800/50 hover:odd:bg-slate-100/75 hover:odd:dark:bg-slate-800/75 even:bg-gray-100/25 even:dark:bg-gray-800/25 hover:even:bg-gray-50/25 hover:even:dark:bg-gray-700/25 -->
                                    <td class="px-3 py-2 text-sm font-normal max-w-sm whitespace-nowrap overflow-x-auto">{k}</td>
                                    <td class="px-3 py-2 text-sm font-extralight whitespace-nowrap max-w-[50%] overflow-x-auto">{v}</td>
                                </tr>
                            {/each}
                        {/if}
                    </tbody>
                </table>
            </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 max-h-96 mb-8">
            <!-- Email -->
            <div class="rounded bg-white dark:bg-zinc-800 shadow-sm px-3 py-2 mt-4 col-span-1 xl:col-span-2 ">
                <div class="flex">
                    <h2 class="grow">Email</h2>
                    <div class="relative">
                        <button onclick={ showEmailViewOptions = !showEmailViewOptions} class="relative px-2 py-1 border rounded border-slate-200 dark:border-slate-700 font-light min-w-32">
                            <span class="flex items-center gap-1">
                                <p class="grow text-left">{ emailState } </p>
                                <svg xmlns="http://www.w3.org/2000/svg" class="ionicon w-4 h-4" viewBox="0 0 512 512"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="24" d="M112 184l144 144 144-144"/></svg>
                            </span>
                        </button>
                        {#if showEmailViewOptions }
                        <div class="absolute top-0 left-0 mt-8 px-2 py-1 bg-white dark:bg-zinc-900 flex flex-col p-1 gap-1 justify-start text-left border rounded border-slate-200 dark:border-slate-700 divide-y divide-slate-200 min-w-32">
                            <button onclick={ () => updateEmailView('Rendered') } class="hover:bg-zinc-50 hover:dark:bg-zinc-800 cursor-pointer">Rendered</button>
                            <button onclick={ () => updateEmailView('Plaintext') } class="hover:bg-zinc-50 hover:dark:bg-zinc-800 cursor-pointer">Plaintext</button>
                            <button onclick={ () => updateEmailView('HTML') } class="hover:bg-zinc-50 hover:dark:bg-zinc-800 cursor-pointer">HTML</button>
                        </div>
                        {/if}
                    </div>


                </div>
                <div bind:this={emlContainer} class="border rounded border-slate-200 dark:border-slate-700 my-2 font-light text-sm max-h-80 overflow-auto">
                    <!--{@html context.eml.body}-->
                </div>
            </div>

            <!-- Indicators-->
            <div class="rounded bg-white dark:bg-zinc-800 shadow-sm px-3 py-2 mt-4">
                <div class="flex">
                    <h2 class="grow">Indicators</h2>
                    
                </div>
                <div class="border rounded border-slate-200 dark:border-slate-700 my-2 font-light text-sm max-h-80 overflow-auto p-2">
                    <h3>Domains</h3>
                    <ul>
                        {#each context.eml.indicators.domains as domain}
                        <li>{domain}</li>
                        {/each}
                    </ul>
                    <h3>URLs</h3>
                    <ul>
                        {#each context.eml.indicators.urls as url}
                        <li>{url}</li>
                        {/each}
                    </ul>
                    <h3>Emails</h3>
                    <ul>
                        {#each context.eml.indicators.emails as email}
                        <li>{email}</li>
                        {/each}
                    </ul>
                </div>
            </div>
            <!--<div class="border rounded-lg border-slate-200 dark:border-slate-700 p-2 my-2 max-h-80 overflow-auto">-->
                
            
        </div>
        <div class="h-2"></div>
        <!--<div>{eml.body}</div>-->
    {:else if context.eml.error}
        <p>Error: {context.eml.error}</p>
    {:else }
        <!-- No Analysis Yet -->
         <div class="flex flex-col justify-center items-center h-full gap-2 ">
            <button onclick={openFile} class="mx-auto rounded bg-blue-500 hover:bg-blue-600 text-white px-2 py-1 cursor-pointer">Open .eml</button>
            <p class="text-center text-sm text-zinc-500">Open a Email to start the analysis.</p>
        </div>
    {/if}



    <!--<p>EML: {JSON.stringify(eml)}</p>-->
</div>
