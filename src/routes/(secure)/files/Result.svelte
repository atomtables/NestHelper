<!-- this only exists so i don't reload SSH every time i make a UI change -->
<script>
    import folder from "$lib/assets/folder.png";
    import file from "$lib/assets/file.png";
    import Input from "$lib/components/Input.svelte";
    import Button from "$lib/components/Button.svelte";
    import {alert} from "$lib/components/Dialog.svelte";
    import {app, filesystem} from "$lib/state/states.svelte.js";
    import {Command} from "$lib/conn/Command.js";
    import Editor from "$lib/components/Editor.svelte";
    import File from "#root/Volumes/TheEpcodeFiles/NestHelper/src/routes/(secure)/files/File.svelte";

    const sort = children => Object.entries(children)
        .sort(([a,x], [b,y]) => x.type === 'folder' && y.type === 'folder' ? a.localeCompare(b) :
            x.type === 'folder' ? -1 : y.type === 'folder' ? 1 : a.localeCompare(b))

    // noinspection JSUnresolvedReference
    let currentFolder = $derived.by(() => {
        let folder = filesystem.value.files;
        for (let i = 1; i < filesystem.value.currentFolder.length; i++) {
            console.log(`Navigating to ${filesystem.value.currentFolder[i]}`);
            folder = folder.children[filesystem.value.currentFolder[i]];
        }
        return sort(folder.children);
    })
    let {currentFilePath = $bindable(), selectedFile = $bindable()} = $props()

    const read = file => `python3 -c 'with open("${file}", "rb") as f:print([int(x) for x in f.read()])'`;

    const loadFile = async () => {
        let filePath = filesystem.value.currentFolder.join('/') + '/' + selectedFile[0];
        app.value.status = `Loading file: ${filePath}`;
        try {
            let output = await Command(read(filePath))
            filesystem.value.fileData[filePath] = {
                original: new Uint8Array(JSON.parse(output.stdout)),
                modified: new Uint8Array(JSON.parse(output.stdout)) // two instances
            }
            currentFilePath = filePath;
            app.value.status = `File loaded: ${filePath}`;if (!filesystem.value.currentFolder) {
                // noinspection JSValidateTypes
                filesystem.value.currentFolder = [folder];
            }
        } catch (e) {
            console.error("Error loading file:", e);
            await alert(
                "Error loading file",
                `There was an error loading your file.
                ${e instanceof Error ? `The error returned the following message: ${e.message}` : ''}
                ${e.event === "CommandOutput" ? `The command exited with return code ${e.code} and returned the following output:
                <br>
                STDOUT: <pre class="">${e.stdout}</pre><br>
                STDERR: <pre class="">${e.stderr}</pre>` : ""}
                ${e instanceof TypeError ? "Please choose the correct type for the file you wish to view." : ""}
            `);
        } finally {
            app.value.status = "";
        }
    }
    const unloadFile = () => {
        delete filesystem.value.fileData[currentFilePath];
        currentFilePath = null;
    }

    let folderPath = $derived(filesystem.value.currentFolder)
</script>

<div class="w-full h-full flex flex-row flex-nowrap inset-0">
    <div class="flex-1/3 flex flex-col items-start w-full gap-0 p-4 bg-purple-900 rounded-2xl m-4 overflow-auto scroll-auto">
        {#if folderPath.length > 1}
            <button class="text-xs text-neutral-300 hover:underline" onclick={() => folderPath.pop()}>
                ^ up
            </button>
        {/if}
        <div class="flex items-center max-w-full">
            <div class="truncate">
                {folderPath.slice(0, -1).join('/')}
            </div>
            {#if folderPath.length > 1}<span>/</span>{/if}
            <div class="flex-shrink-0">
                {folderPath[folderPath.length - 1]}
            </div>
        </div>
        {#each currentFolder as [key, value]}
            <div
                    class="pl-2 max-w-full w-full flex flex-row items-center gap-3
                         hover:bg-purple-800 active:bg-purple-700 transition-all
                           rounded-lg cursor-pointer p-1"
                    onclick={value.type === 'folder' ? () => {
                        folderPath.push(key);
                    } : () => {
                        selectedFile = [key, value];
                        if (filesystem.value.fileData[folderPath.join('/') + '/' + key]) {
                            currentFilePath = folderPath.join('/') + '/' + key;
                        } else {
                            currentFilePath = null;
                        }
                    }}
            >
                {#if value.type === 'folder'}
                    <img src={folder} alt="folder" class="w-4">
                    <div class="text-ellipsis overflow-x-clip whitespace-nowrap">{key}</div>
                {:else}
                    <img src={file} alt="folder" class="w-4">
                    <div class="text-ellipsis overflow-x-clip whitespace-nowrap">{key}</div>
                {/if}
            </div>
        {/each}
    </div>
    <div class="flex-3/4 flex flex-col justify-center items-center m-4 overflow-auto scroll-auto">
        {#if currentFilePath}
            <!--suppress JSUnresolvedReference -->
            {#key currentFilePath}
                <File filename={selectedFile[0]} filePath={currentFilePath} nvm={unloadFile} />
            {/key}
        {:else}
            {#if selectedFile}
                <img src={file} alt="file" class="w-16 mb-4">
                <div class="text-2xl font-bold">{selectedFile[0]}</div>
                <div>Size: {selectedFile[1].size} bytes</div>
                <div class="pt-4 flex flex-row gap-2 items-center justify-center">
                    <Input class="appearance-none w-full" type="dropdown" name="File type" elements={["Text", "Image", "Raw Bytes"]}/>
                    <Button class="w-full" onclick={loadFile}>Load file</Button>
                </div>
            {:else}
                <div class="text-2xl font-bold">No file selected</div>
                <div>Select a file to view its information.</div>
            {/if}
        {/if}
    </div>
</div>