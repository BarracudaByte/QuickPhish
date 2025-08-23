<script>
    // @ts-nocheck
    import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
    import JsonViewer from './JsonViewer.svelte';

    export let data;
    export let parentPath = '';

    const isObject = typeof data === 'object' && data !== null && !Array.isArray(data);
    const isArray = Array.isArray(data);

    async function copyToClipboard(text) {
        try {
            await writeText(text);
            console.log(`Path ${text} copied successfully!`);
        } catch (err) {
            console.error('Failed to copy text using fallback:', err); 
        }
    }
</script>


<div class="json-viewer space-y-1 pl-2">
    {#if data}
        {#each Object.entries(data) as [key, value]}
            <div class="{typeof value === 'object' && value !== null ? '' : 'flex'} items-start gap-1">
                <!-- Render the clickable key -->
                <button 
                    class="font-mono cursor-copy text-sm text-sky-500 hover:text-sky-700 transition-colors duration-200"
                    on:click={() => copyToClipboard(parentPath ? `${parentPath}.${key}` : key)}
                    title="Click to copy path"
                >
                    {key}:
                </button>
                <br>

                {#if typeof value === 'object' && value !== null}
                    <!-- If the value is a nested object or array, render a new JsonViewer component recursively -->
                    <!--<span class="text-sm text-gray-400">{isObject ? '{' : '['}</span>-->
                    <div class="space-y-1 pl-2">
                        <JsonViewer data={value} parentPath={parentPath ? `${parentPath}.${key}` : key} />
                    </div>
                    <!--<span class="text-sm text-gray-400">{isObject ? '}' : ']'}</span>-->
                {:else}
                    <!-- If the value is a primitive, just display it -->
                    <span class="text-sm {typeof value === 'string' ? 'text-green-500' : 'text-purple-500 font-mono'} line-clamp-3">
                        {typeof value === 'string' ? `"${value}"` : String(value)}
                    </span>
                {/if}
            </div>
        {/each}
    {/if}
</div>