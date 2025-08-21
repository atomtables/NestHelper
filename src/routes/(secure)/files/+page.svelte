<script>
    import {Command} from "$lib/conn/Command.js";
    import {onMount} from "svelte";
    import {wait} from "$lib/components/Dialog.svelte";
    import Spinner from "$lib/components/Spinner.svelte";
    import {app, auth, filesystem, save} from "$lib/state/states.svelte.ts";
    import Result from "./Result.svelte";

    let folder = `/home/${auth.value.username}`
    let ignore = ["node_modules","venv",".git",".local",".cache","__pycache__"]
    let command = `M="folder"
L=print
G="type"
D="children"
import json,os as A
H="${folder}"
N=[${ignore.map(n => `"${n}"`).join(",")}]
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
	for K in O:B[K]={G:\"file\",\"size\":A.path.getsize(A.path.join(I,K))}
L(json.dumps(E))`

    let promise = null;

    onMount(() => {
        app.value.status = "Loading files...";
        console.log(filesystem.value)
        if (new Date() - (filesystem.value?.lastUpdated || new Date(0)) > 1000 * 60 * 5) {
            promise = Command(`python3 -c '${command}'`)
                .then(async res => {
                    app.value.status = `Ignoring ${ignore.length} patterns`;
                    filesystem.set = true
                    // noinspection JSValidateTypes (esp because ts has no problem with this)
                    filesystem.value = {
                        files: JSON.parse(res?.stdout),
                        fileData: {},
                        lastUpdated: new Date(),
                        currentFolder: [folder]
                    }
                    await save(filesystem)
                })
                .catch(error => {
                    console.error("Error executing command:", error);
                    throw error;
                });
            wait(promise, "Loading files", "This is necessary to display the current filesystem and update the state.")
        } else {
            promise = Promise.resolve();
            app.value.status = `Ignoring ${ignore.length} patterns`;
        }
    })

</script>

{#if filesystem.value.files}
    <Result />
{:else}
    {#await promise}
        <div class="flex flex-row items-center justify-center w-full h-full">
            <Spinner />
        </div>
    {:catch error}
        <div class="text-red-500">Error: {error.message}</div>
    {/await}
{/if}