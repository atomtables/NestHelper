<script>
    import Editor from "$lib/components/Editor.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import {highlighting, supportedLanguages} from "$lib/helpers/monaco.js";

    let {
        file = $bindable(),
        filename,
        folderPath
    } = $props()

    let currentLanguage = $state(highlighting(filename));
</script>

<div class="flex flex-col w-full h-full gap-2">
    <div class="flex flex-row items-center justify-between gap-2">
        <div class="text-sm text-neutral-300">{folderPath}</div>
        <Dropdown direction="right" class="[&]:p-0 [background-color:unset]! hover:underline text-sm font-light" items={supportedLanguages()}>{currentLanguage}</Dropdown>
    </div>
    <Editor bind:value={file} language={currentLanguage}/>
</div>