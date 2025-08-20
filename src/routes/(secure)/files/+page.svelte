<script>
    import {Command} from "$lib/conn/Command.js";
    import {onMount} from "svelte";
    import Spinner from "$lib/components/Spinner.svelte";
    import {app, auth} from "$lib/state/states.svelte.ts";
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
        promise = Command(`python3 -c '${command}'`)
            .then(res => {
                app.value.status = `Ignoring ${ignore.length} patterns`;
                return JSON.parse(res?.stdout)
            })
            .catch(error => {
                console.error("Error executing command:", error);
                throw error;
            });
    })

</script>

{#await promise}
    <Spinner />
{:then result}
    <Result {result} dir={folder} />
{:catch error}
    <div class="text-red-500">Error: {error.message}</div>
{/await}