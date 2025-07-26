<script>
// @ts-nocheck
    import { open } from '@tauri-apps/plugin-dialog';
    import { listen } from '@tauri-apps/api/event';
    
    listen('tauri://file-drop', event => {
        console.log(`File Drop Event! ${event}`)
    })

    async function openFile() {
        const file = await open({
            multiple: false,
            directory: false,
        });
        console.log(`Open file: ${file}`);
    }

    function dropHandler(ev) {
        console.log("File(s) dropped");

        // Prevent default behavior (Prevent file from being opened)
        ev.preventDefault();

        if (ev.dataTransfer.items) {
            // Use DataTransferItemList interface to access the file(s)
            [...ev.dataTransfer.items].forEach((item, i) => {
                // If dropped items aren't files, reject them
                if (item.kind === "file") {
                    const file = item.getAsFile();
                    console.log(`… file[${i}].name = ${file.name}`);
                }
            });
        } else {
            console.log("Else Case");
            // Use DataTransfer interface to access the file(s)
            [...ev.dataTransfer.files].forEach((file, i) => {
                console.log(`… file[${i}].name = ${file.name}`);
            });
        }
    }

    function dragOverHandler(ev) {
        console.log("File(s) in drop zone");

        // Prevent default behavior (Prevent file from being opened)
        ev.preventDefault();
    }

</script>
<!-- on:drop={dropHandler} on:dragover={dragOverHandler}  -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div id="drop_zone"  class="cursor-copy border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-sm bg-zinc-100 dark:bg-zinc-800 px-2 py-8 w-full text-center">
    <p class="p-2 bg-blue-500/25 mx-auto w-fit rounded-sm text-slate-800 dark:text-slate-200 select-none cursor-default">Drag &amp; Drop .eml</p>
    <p class="text-xs font-bold text-slate-700 dark:text-slate-300">or</p>
    <button on:click={openFile} class="text-blue-500 hover:text-blue-600 cursor-pointer">Browse</button>
</div>