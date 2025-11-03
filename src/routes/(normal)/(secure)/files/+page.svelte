<script>
    import { Command } from '$lib/conn/Command.ts';
    import { onMount } from 'svelte';
    import { wait } from '$lib/components/generic/Dialog.svelte';
    import Spinner from '$lib/components/generic/Spinner.svelte';
    import { app, auth, currentFlow, filesystem, save } from '$lib/state/states.svelte.ts';
    import { slide } from 'svelte/transition';
    import { alert, confirm, prompt } from '$lib/components/generic/Dialog.svelte';
    import { open as openDialog } from '@tauri-apps/plugin-dialog';
    import Result from './Result.svelte';
    import { emit } from '@tauri-apps/api/event';
    import { readFile } from '@tauri-apps/plugin-fs';
    import Workflow from '$lib/conn/Workflow.svelte';

    let folder = `/home/${auth.value.username}`;
    let ignore = ['node_modules', 'venv', '.git', '.local', '.cache', '__pycache__'];
    let command = `M="folder"
L=print
G="type"
D="children"
import json,os as A
H="${folder}"
N=[${ignore.map((n) => `"${n}"`).join(',')}]
E={G:M,D:{}}
for(I,Q,O)in A.walk(H):
	F=A.path.relpath(I,H)
	if F==\".\":B=E[D]
	else:
		J=True;P=F.split(A.sep);B=E[D]
		for C in P:
			if C in N:break
			if C not in B:B[C]={G:M,D:{}}
			B=B[C][D]
		else:J=False
		if J:continue
	for K in O:
		if not A.path.exists(A.path.join(I,K)):continue
		B[K]={G:"file","size":A.path.getsize(A.path.join(I,K))}
L(json.dumps(E))`;

    let currentFilePath = $state();
    let selectedFile = $state();

    let promise = $state(null);

    const refresh = () => {
        let flow = currentFlow.value = new Workflow(
            [
                {
                    task: 'Fetch files',
                    command: `python3 -c '${command}'`,
                },
                {
                    task: 'Update file UI',
                    frontend: true,
                    promise: async (res) => {
                        res = res[1]
                        app.value.status = `Ignoring ${ignore.length} patterns`;
                        filesystem.set = true; 
                        console.log(res);
                        let newfiles = JSON.parse(res?.stdout);
                        let filesWithNoExistence = [];
                        for (const filePath in filesystem.value?.fileData) {
                            if (filesystem.value.fileData[filePath]?.newFile) continue; // skip new files
                            if (filesystem.value.fileData[filePath]?.deletedFile) continue; // skip deleted files
                            // If the file no longer exists, remove it from fileData
                            let path = filePath.replace(folder + '/', '').split('/');
                            let curr = newfiles;
                            for (const part of path) {
                                if (curr && curr.type === 'folder') {
                                    curr = curr.children[part];
                                } else if (curr && curr.type === 'file') {
                                    break;
                                } else {
                                    curr = null;
                                    break;
                                }
                            }
                            if (!curr) {
                                filesWithNoExistence.push(filePath);
                            }
                        }
                        if (filesWithNoExistence.length > 0) {
                            let [result] = await confirm(
                                'Deleted files detected',
                                `The following files no longer exist and will be removed from the filesystem state: <br> ${filesWithNoExistence.map((f) => `<b>${f}</b>`).join('<br>')} <br> Would you like to update, resulting in the deletion of your modified file changes?`
                            );
                            if (result) {
                                for (const filePath of filesWithNoExistence) {
                                    delete filesystem.value.fileData[filePath];
                                }
                                filesystem.value.files = newfiles;
                                filesystem.value.lastUpdated = new Date();
                                await save(filesystem);
                                selectedFile = null;
                                currentFilePath = null;
                            }
                        } else {
                            filesystem.value.files = newfiles;
                            filesystem.value.lastUpdated = new Date();
                            await save(filesystem);
                        }
                        (() => {
                            for (const key in filesystem.value.fileData) {
                                let arr1 = filesystem.value.fileData[key].original;
                                let arr2 = filesystem.value.fileData[key].modified;
                                if (!(arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]))) {
                                    filesystem.value.filesWereModified = true;
                                    return;
                                }
                            }
                            filesystem.value.filesWereModified = false;
                        })();
                    },
                },
            ],
            'Fetch updated file structures'
        );
        flow.start().then(() => {
            promise = flow.promise;
            flow.promise.catch((error) => {
                console.error('Error executing command:', error);
                alert(
                    'Failed to load files',
                    `There was an error loading the files. The server returned the following outputs: <div class="max-h-96 overflow-scroll"><br> STDOUT: <p class="text-red-500">${flow.tasks.at(0)?.stdout.replaceAll('\n', '<br>')}</p> <br> STDERR: <p class="text-red-500">${flow.tasks.at(0)?.stderr.replaceAll('\n', '<br>')}</p> <br></div>`
                );
            });
        })
    };
    const newfile = async () => {
        let folderPath = filesystem.value.currentFolder.join('/');
        let filename = await prompt('Enter the name of the new file (including extension):', 'newfile.txt');
        if (!filename) return;
        if (filename.includes('/')) {
            await alert('Invalid filename', 'The filename cannot contain slashes.');
            return;
        }
        if (filesystem.value.files.children[filename]) {
            let result = await confirm('File already exists', `The file <b>${filename}</b> already exists in the current folder. Would you like to overwrite it?`);
            if (!result) return;
        }
        filesystem.value.fileData[`${folderPath}/${filename}`] = {
            original: new Uint8Array(),
            modified: new Uint8Array(),
            newFile: true,
        };
        let cur = filesystem.value.files;
        for (const part of filesystem.value.currentFolder) {
            if (part === filesystem.value.currentFolder[0]) continue;
            if (!cur.children[part]) {
                await alert('Error', `The folder <b>${part}</b> does not exist in the filesystem. Please refresh the filesystem.`);
                return;
            } else if (cur.children[part].type !== 'folder') {
                await alert('Error', `The path <b>${part}</b> is not a folder in the filesystem. Please refresh the filesystem.`);
                return;
            }
            cur = cur.children[part];
        }
        cur.children[filename] = {
            type: 'file',
            size: 0,
        };
        currentFilePath = `${folderPath}/${filename}`;
        console.log(filename);
        selectedFile = [filename, filesystem.value.fileData[`${folderPath}/${filename}`]];
        console.log(selectedFile);
    };
    const uploadfile = async () => {
        const file = await openDialog({
            multiple: false,
            directory: false,
        });
        if (!file) return;
        const read = await readFile(file);
        let filename = file.split('/').pop();
        let folderPath = filesystem.value.currentFolder.join('/') + '/' + filename;
        if (filesystem.value.fileData[folderPath]) {
            let [result] = await confirm('File already exists', `The file <b>${filename}</b> already exists in the current folder. Would you like to overwrite it?`);
            if (!result) return;
        }
        let cur = filesystem.value.files;
        for (const part of filesystem.value.currentFolder) {
            if (part === filesystem.value.currentFolder[0]) continue;
            if (!cur.children[part]) {
                await alert('Error', `The folder <b>${part}</b> does not exist in the filesystem. Please refresh the filesystem.`);
                return;
            } else if (cur.children[part].type !== 'folder') {
                await alert('Error', `The path <b>${part}</b> is not a folder in the filesystem. Please refresh the filesystem.`);
                return;
            }
            cur = cur.children[part];
        }
        cur.children[filename] = {
            type: 'file',
            size: read.byteLength,
        };
        filesystem.value.fileData[folderPath] = {
            original: new Uint8Array(),
            modified: new Uint8Array(read),
            newFile: true,
        };
    };

    const manageexclusionrules = async () => {};

    let mounted = $state(false);
    onMount(() => {
        app.value.status = 'Loading files...'; 
        try {
            if (filesystem.value) {
                filesystem.value.fileData = filesystem.value.fileData || {};
                filesystem.value.currentFolder = [folder];
            }
        } catch {}
        if (!filesystem.value?.lastUpdated || new Date() - (new Date(filesystem.value?.lastUpdated) || new Date(0)) > 1000 * 60 * 5) {
            refresh();
        } else {
            promise = Promise.resolve();
            app.value.status = `Ignoring ${ignore.length} patterns`;
        }
        mounted = true;

        app.value.pageActions = [
            ['refresh', refresh],
            ['new file', newfile],
            ['upload file', uploadfile],
            ['manage exclusion rules', manageexclusionrules],
        ];

        let int = setInterval(() => {
            refresh();
        }, 300000);

        return () => {
            clearInterval(int);
            mounted = false;
        };
    });
</script>

<div class="w-full h-full" transition:slide>
    {#if filesystem.value.files && mounted}
        <Result bind:currentFilePath bind:selectedFile />
    {:else}
        {#await promise}
            <div class="flex flex-row items-center justify-center w-full h-full">
                <Spinner />
                <div>Loading files...</div>
            </div>
        {:catch error}
            <div class="text-red-500">Error: {error.message}</div>
        {/await}
    {/if}
</div>
