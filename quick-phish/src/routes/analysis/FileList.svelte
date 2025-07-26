<script>
// @ts-nocheck
    import { open } from '@tauri-apps/plugin-dialog';

    async function openFile() {
        const file = await open({
            multiple: false,
            directory: false,
        });
        console.log(`Open file: ${file}`);
    }
</script>

<div class="flex flex-col gap-2 p-2 font-light text-sm bg-zinc-50 dark:bg-zinc-900 text-black dark:text-white h-screen min-w-32 max-w-48 border-zinc-100 dark:border-black border-r">
    <div class="grow">
        <div>File 1</div>
        <div>File 2</div>
        <div>File 3</div>
    </div>
    <button on:click={openFile} class="text-blue-500 hover:text-blue-600 cursor-pointer">Open .eml</button>
</div>