<script>
    import Editor from "$lib/components/Editor.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import {highlighting, supportedLanguages} from "$lib/helpers/monaco.ts";
    import {onMount} from "svelte";
    import {alert} from "$lib/components/Dialog.svelte";
    import {filesystem} from "$lib/state/states.svelte.ts";
    import { event } from '@tauri-apps/api';
    import { confirm } from "$lib/components/Dialog.svelte";

    let {
        filePath
    } = $props()

    let file = $state()
    onMount(() => {
        try {
            // lazy ass code writing
            file = `data:image/unknown;base64,${filesystem.value.fileData[filePath].original.toBase64()}`
        } catch (e) {
            console.error("Error decoding file:", e);
            alert("Error", `There was an error decoding the file: ${e.message}. Ensure you selected the correct file type.`);
            file = null;
        }
    })
</script>

{#if file != null}
    <div class="flex flex-col w-full h-full gap-2">
        <div class="flex flex-row items-center justify-between gap-2">
            <div class="text-sm text-neutral-300">{filePath}</div>
            <div>(read only)</div>
        </div>
        <div class="flex flex-col max-w-full min-h-0">
            <img src={file} alt={filePath} class="object-scale-down" />
        </div>
    </div>
{/if}