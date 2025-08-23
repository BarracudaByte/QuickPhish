<script>
// @ts-nocheck
    import { invoke } from '@tauri-apps/api/core';

    let whitelist = $state("");
    let blacklist = $state("");
    let loadingWhitelist = $state(false);
    let updatedWhitelist = $state(false);
    let loadingBlacklist = $state(false);
    let updatedBlacklist = $state(false);

    invoke('get_lists').then((data) => { 
        whitelist = data.whitelist.content; 
        blacklist = data.blacklist.content;
        console.log(data);
    });

    async function updateWhitelist() {
        loadingWhitelist = true;
        invoke('update_list', { list: "whitelist", content: whitelist }).then((ok) => { 
            console.log(`Update Whitelist: ${ok}`);
            loadingWhitelist = false;
            if (ok) {
                updatedWhitelist = true;
            }
        });
    }

    function whitelistChanged() {
        updatedWhitelist = false;
        console.log("Input has changed...");
    }


    async function updateBlacklist() {
        updatedBlacklist = true;
        invoke('update_list', { list: "blacklist", content: blacklist }).then((ok) => { 
            console.log(`Update Blacklist: ${ok}`);
            loadingBlacklist = false;
            if (ok) {
                updatedBlacklist = true;
            }
        });
    }

    function blacklistChanged() {
        updatedBlacklist = false;
        console.log("Input has changed...");
    }

</script>

<div class="m-2 h-dvh">
    <h1>Lists</h1>
    <p>Black- and Whitelists to always block or allow specific IOCs. Add each value in a new line.</p>
    <h2 class="mt-4">Whitelist</h2>
    <textarea type="text" bind:value={whitelist} oninput={whitelistChanged} placeholder="Whitelist Domains / URLs / Emails" class="flex h-auto min-h-[144px] w-full px-3 py-2 text-sm border rounded-md border-slate-200 dark:border-slate-700 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-slate-300 focus:dark:ring-slate-700 disabled:cursor-not-allowed disabled:opacity-50"></textarea>
    <div class="flex flex-row-reverse my-2">
        {#if loadingWhitelist}
            <div class="animate-spin inline-block size-6 border-3 border-current border-t-transparent text-blue-600 rounded-full dark:text-blue-500" role="status" aria-label="loading">
                <span class="sr-only">Loading...</span>
            </div>
        {:else if updatedWhitelist }
            <p class="text-green-500">&#10003;</p>
        {:else}
            <button onclick={updateWhitelist} class="rounded bg-blue-500 hover:bg-blue-600 text-white px-2 py-1 cursor-pointer">Update</button>
        {/if}
    </div>

    <h2 class="mt-4">Blacklist</h2>
    <textarea type="text" bind:value={blacklist} oninput={blacklistChanged} placeholder="Blacklist Domains / URLs / Emails" class="flex h-auto min-h-[144px] w-full px-3 py-2 text-sm border rounded-md border-slate-200 dark:border-slate-700 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-slate-300 focus:dark:ring-slate-700 disabled:cursor-not-allowed disabled:opacity-50"></textarea>
    <div class="flex flex-row-reverse my-2">
        {#if loadingBlacklist}
            <div class="animate-spin inline-block size-6 border-3 border-current border-t-transparent text-blue-600 rounded-full dark:text-blue-500" role="status" aria-label="loading">
                <span class="sr-only">Loading...</span>
            </div>
        {:else if updatedBlacklist }
            <p class="text-green-500">&#10003;</p>
        {:else}
            <button onclick={updateBlacklist} class="rounded bg-blue-500 hover:bg-blue-600 text-white px-2 py-1 cursor-pointer">Update</button>
        {/if}
    </div>


    
</div>