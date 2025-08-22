<script>
    import Editor from "$lib/components/Editor.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import {highlighting, supportedLanguages} from "$lib/helpers/monaco.js";
    import {onMount} from "svelte";
    import {alert} from "$lib/components/Dialog.svelte";
    import {filesystem} from "$lib/state/states.svelte.js";
    import { event } from '@tauri-apps/api';
    import { confirm } from "$lib/components/Dialog.svelte";

    let {
        filename,
        filePath,
        nvm
    } = $props()

    let file = $state()
    onMount(() => {
        try {
            file = new TextDecoder('utf-8', { fatal: true }).decode(filesystem.value.fileData[filePath].modified)
        } catch (e) {
            console.error("Error decoding file:", e);
            alert("Error", `There was an error decoding the file: ${e.message}. Ensure you selected the correct file type.`);
            file = null;
            nvm?.()
        }
    })

    // file => .modified (changes made by user)
    $effect(() => {
        filesystem.value.fileData[filePath].modified = new TextEncoder('utf-8', { fatal: true }).encode(file)
    })
    $effect(() => {
        let arr1 = filesystem.value.fileData[filePath].original;
        let arr2 = filesystem.value.fileData[filePath].modified;
        console.log(arr1, arr2)
        if (!(arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]))) {
            filesystem.value.filesWereModified = true;
        } else {
            for (const key in filesystem.value.fileData) {
                let arr1 = filesystem.value.fileData[key].original;
                let arr2 = filesystem.value.fileData[key].modified;
                if (!(arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]))) {
                    filesystem.value.filesWereModified = true;
                    return;
                }
            }
            filesystem.value.filesWereModified = false;
        }
    })

    let currentLanguage = $state(highlighting(filename));
</script>

{#if file != null}
    <div class="flex flex-col w-full h-full gap-2">
        <div class="flex flex-row items-center justify-between gap-2">
            <div class="text-sm text-neutral-300">{filePath}</div>
            <Dropdown direction="right" class="[&]:p-0 [background-color:unset]! hover:underline text-sm font-light" items={supportedLanguages()}>{currentLanguage}</Dropdown>
        </div>
        <Editor bind:value={file} language={currentLanguage}/>
    </div>
{/if}