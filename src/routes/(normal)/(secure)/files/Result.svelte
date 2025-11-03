<!-- this only exists so i don't reload SSH every time i make a UI change -->
<script lang="ts">
    import folder from '$lib/assets/folder.png';
    import file from '$lib/assets/file.png';
    import Input from '$lib/components/generic/Input.svelte';
    import Button from '$lib/components/generic/Button.svelte';
    import { alert, prompt } from '$lib/components/generic/Dialog.svelte';
    import { app, filesystem, type FileFolder } from '$lib/state/states.svelte';
    import { confirm } from '$lib/components/generic/Dialog.svelte';
    import { Command } from '$lib/conn/Command';
    import File from '$lib/components/viewers/File.svelte';
    import { supportedBinary, supportedImage } from '$lib/helpers/monaco';
    import ImageFile from '$lib/components/viewers/ImageFile.svelte';
    import BinaryFile from '$lib/components/viewers/BinaryFile.svelte';
    import { mount, unmount } from 'svelte';
    import RightClickContext from './RightClickContext.svelte';

    const sort = (children: FileFolder) => Object.entries(children || {}).sort(([a, x], [b, y]) => (x.type === 'folder' && y.type === 'folder' ? a.localeCompare(b) : x.type === 'folder' ? -1 : y.type === 'folder' ? 1 : a.localeCompare(b)));

    let currentFolder = $derived.by(() => {
        let folder = filesystem.value.files;
        for (let i = 1; i < filesystem.value.currentFolder.length; i++) {
            folder = folder.children[filesystem.value.currentFolder[i]];
        }
        return sort(folder.children);
    });
    let { currentFilePath = $bindable(), selectedFile = $bindable() } = $props();
    let selectedFileType: 0 | 1 | 2 = $state(0);

    const read = (file) => `python3 -c 'with open("${file}", "rb") as f:print([int(x) for x in f.read()])'`;

    const loadFile = async () => {
        let filePath = filesystem.value.currentFolder.join('/') + '/' + selectedFile[0];
        if (filesystem.value.fileData[filePath]) {
            console.log($state.snapshot(filesystem.value.fileData[filePath]));
            filesystem.value.fileData[filePath].type = selectedFileType;
        }
        app.value.status = `Loading file: ${filePath}`;
        if (filesystem.value.fileData[filePath]?.deletedFile) {
            await alert('File deleted', 'You have marked this file for deletion. Undo the deletion to load it.');
            app.value.status = '';
            return;
        }
        if (filesystem.value.fileData[filePath]?.newFile) {
            currentFilePath = filePath;
            filesystem.value.fileData[filePath].type = selectedFileType;
            return;
        }
        try {
            let output = await Command(read(filePath));
            let val = filesystem.value.fileData[filePath] || {
                type: selectedFileType,
                original: new Uint8Array(JSON.parse(output.stdout)),
                modified: new Uint8Array(JSON.parse(output.stdout)), // two instances
            };
            val.type = selectedFileType;
            val.original = new Uint8Array(JSON.parse(output.stdout));
            val.modified = new Uint8Array(JSON.parse(output.stdout)); // two instances
            filesystem.value.fileData[filePath] = val;
            currentFilePath = filePath;
            app.value.status = `File loaded: ${filePath}`;
        } catch (e) {
            console.error('Error loading file:', e);
            await alert(
                'Error loading file',
                `There was an error loading your file.
                ${e instanceof Error ? `The error returned the following message: ${e.message}` : ''}
                ${
                    e.event === 'CommandOutput'
                        ? `The command exited with return code ${e.code} and returned the following output:
                <br>
                STDOUT: <pre class="">${e.stdout}</pre><br>
                STDERR: <pre class="">${e.stderr}</pre>`
                        : ''
                }
                ${e instanceof TypeError ? 'Please choose the correct type for the file you wish to view.' : ''}
            `
            );
        } finally {
            app.value.status = '';
        }
    };
    const unloadFile = () => {
        console.error('Had to unload file');
        filesystem.value.fileData[currentFilePath].type = null;
        currentFilePath = null;
    };
    const deleteFile = async () => {
        let result = await confirm('Are you sure you would like to delete this file?', `You are about to delete the file <b>${selectedFile[0]}</b>. Would you like to continue?`);
        if (!result) return;

        let filePath = filesystem.value.currentFolder.join('/') + '/' + selectedFile[0];
        filesystem.value.fileData[filePath] = {
            type: 0,
            original: new Uint8Array([]),
            modified: new Uint8Array([]),
            deletedFile: true,
        };
    };

    let folderPath = $derived(filesystem.value.currentFolder);
    const fileWasModified = (filePath) => {
        if (!filesystem.value.fileData[filePath]) return false;
        let arr1 = filesystem.value.fileData[filePath].original;
        let arr2 = filesystem.value.fileData[filePath].modified;
        return !(arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]));
    };

    const navigateToNew = (key, value) => {
        (value.type === 'folder'
            ? () => {
                  folderPath.push(key);
              }
            : () => {
                  selectedFile = [key, value];
                  selectedFileType = supportedBinary(key) ? 2 : supportedImage(key) ? 1 : 0;
                  if (filesystem.value.fileData[folderPath.join('/') + '/' + key] && !filesystem.value.fileData[folderPath.join('/') + '/' + key]?.deletedFile) {
                      currentFilePath = folderPath.join('/') + '/' + key;
                  } else {
                      currentFilePath = null;
                  }
              })();
    };

    const rightClickMenu = async (e, key, value) => {
        e.preventDefault();
        let element = document.createElement('div');
        let component: RightClickContext;
        let props = $state({
            file: [key, value],
            mouseX: e.clientX,
            mouseY: e.clientY,
            options: ['Rename'],
            onoptionclicked: async (i) => {
                console.log(folderPath.join('/'), key, value);
                unmount(component);
                if (i === 0) {
                    let newName = await prompt('Rename file', 'Enter the new name you would like to give for the file ' + key);
                    if (newName) {
                        let val = filesystem.value.fileData[folderPath.join('/') + `/${key}`] || {
                            type: null,
                            original: new Uint8Array([]),
                            modified: new Uint8Array([]),
                            renamedFile: [true, filesystem.value.currentFolder.join('/') + '/' +newName],
                        };
                        val.renamedFile = [true, filesystem.value.currentFolder.join('/') + '/' +newName];
                        filesystem.value.fileData[folderPath.join('/') + `/${key}`] = val;
                    }
                }
            },
        });
        document.body.appendChild(element);
        component = mount(RightClickContext, {
            props,
            target: element,
        });
    };
</script>

<div class="w-full h-full flex flex-row flex-nowrap inset-0">
    <div class="flex-1/3 flex flex-col items-start w-full gap-0 p-4 bg-purple-900 rounded-2xl m-4 overflow-auto scroll-auto">
        {#if folderPath.length > 1}
            <button class="text-xs text-neutral-300 hover:underline" onclick={() => folderPath.pop()}> ^ up </button>
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
            <button
                class="pl-2 max-w-full w-full flex flex-row items-center gap-3
                         hover:bg-purple-800 active:bg-purple-700 transition-all
                           {fileWasModified(folderPath.join('/') + `/${key}`) && 'text-amber-500'}
                           {filesystem.value.fileData[folderPath.join('/') + `/${key}`]?.newFile && 'text-green-500'}
                           {filesystem.value.fileData[folderPath.join('/') + `/${key}`]?.deletedFile && 'text-gray-500'}
                           {filesystem.value.fileData[folderPath.join('/') + `/${key}`]?.renamedFile?.[0] && 'text-yellow-500'}
                           rounded-lg cursor-pointer p-1"
                onclick={() => navigateToNew(key, value)}
                oncontextmenu={(e) => rightClickMenu(e, key, value)}
            >
                {#if value.type === 'folder'}
                    <img src={folder} alt="folder" class="w-4" />
                    <div class="text-ellipsis overflow-x-clip whitespace-nowrap">{key}</div>
                {:else}
                    <img src={file} alt="folder" class="w-4" />
                    <span class="w-full flex flex-row justify-between text-ellipsis overflow-x-clip whitespace-nowrap">
                        <span class="text-ellipsis overflow-x-clip whitespace-nowrap">{key}</span>
                        <span class="flex flex-row">
                            {#if filesystem.value.fileData[folderPath.join('/') + `/${key}`]?.renamedFile?.[0]}
                            <span class="text-yellow-500 font-bold">R</span>
                        {/if}
                        {#if filesystem.value.fileData[folderPath.join('/') + `/${key}`]?.newFile}
                            <span class="text-green-500 font-bold">N</span>
                        {/if}
                        {#if filesystem.value.fileData[folderPath.join('/') + `/${key}`]?.deletedFile}
                            <span class="text-gray-500 font-bold">D</span>
                        {/if}
                        {#if fileWasModified(folderPath.join('/') + `/${key}`)}
                            <span class="text-amber-500 font-bold">M</span>
                        {/if}
                        </span>
                    </span>
                {/if}
            </button>
        {/each}
    </div>
    <div class="flex-3/4 w-full flex flex-col justify-center items-center m-4 overflow-auto scroll-auto">
        {#if currentFilePath && filesystem.value.fileData[currentFilePath]?.type != null}
            <!--suppress JSUnresolvedReference -->
            {#key currentFilePath}
                {#if filesystem.value.fileData[currentFilePath].type === 0}
                    <File filename={selectedFile[0]} filePath={currentFilePath} nvm={unloadFile} />
                {:else if filesystem.value.fileData[currentFilePath].type === 1}
                    <ImageFile filePath={currentFilePath} filename={selectedFile[0]} nvm={unloadFile} />
                {:else if filesystem.value.fileData[currentFilePath].type === 2}
                    <BinaryFile filePath={currentFilePath} filename={selectedFile[0]} nvm={unloadFile} />
                {:else}
                    {filesystem.value.fileData[currentFilePath].type}
                {/if}
            {/key}
        {:else if selectedFile}
            {@const fileData = filesystem.value.fileData[folderPath.join('/') + `/${selectedFile[0]}`]}
            <img src={file} alt="file" class="w-16 mb-4" />
            <div class="text-2xl font-bold">{selectedFile[0]}</div>
            {#if fileData?.renamedFile?.[0]}
                <div class="text-yellow-500 font-bold">Renamed to {fileData.renamedFile[1]}</div>
            {/if}
            {#if !fileData?.newFile}
                <div>Size: {selectedFile[1].size} bytes</div>
            {:else}
                <div class="text-green-500 py-2">New file. Save changes to add it.</div>
            {/if}
            {#if fileData?.deletedFile}
                <div class="text-red-500 py-2">This file is marked for deletion. Save changes to delete it.</div>
                <div>
                    <Button destructive class="text-sm" onclick={() => (fileData.deletedFile = false)}>Undo deletion</Button>
                </div>
            {:else}
                <div class="flex flex-row pt-4 gap-2 items-center justify-center w-96">
                    <Input class="appearance-none w-full flex-2/4" type="dropdown" name="File type" elements={['Text', 'Image', 'Raw Bytes']} bind:value={selectedFileType} />
                    <Button class="w-full flex-1/2" onclick={loadFile}>Load file</Button>
                </div>
                <div>
                    {#if !fileData?.newFile}
                        <Button destructive class="text-sm mt-4" onclick={deleteFile}>Delete file</Button>
                    {:else if fileData?.renamedFile?.[0]}
                        <Button
                            destructive
                            class="text-sm mt-4"
                            onclick={() => {
                                fileData.renamedFile = [false, ''];
                            }}>Undo rename</Button
                        >
                    {:else}
                        <Button
                            destructive
                            class="text-sm mt-4"
                            onclick={() => {
                                let folder = filesystem.value.files;
                                for (let i = 1; i < filesystem.value.currentFolder.length; i++) {
                                    folder = folder.children[filesystem.value.currentFolder[i]];
                                }
                                delete folder.children[selectedFile[0]];
                                delete filesystem.value.fileData[folderPath.join('/') + `/${selectedFile[0]}`];
                                selectedFile = null;
                            }}>Remove new file</Button
                        >
                    {/if}
                </div>
            {/if}
        {:else}
            <div class="text-2xl font-bold">No file selected</div>
            <div>Select a file to view its information.</div>
        {/if}
    </div>
</div>
