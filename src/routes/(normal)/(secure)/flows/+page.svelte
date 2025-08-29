<script>
    import { slide } from 'svelte/transition';
    import Button from "$lib/components/Button.svelte";
    import trash from '$lib/assets/trash.png';
    import {auth, currentFlow, save} from '$lib/state/states.svelte.ts'
    import {userflows} from "$lib/state/states.svelte.ts";
    import EditingFlow from "./EditingFlow.svelte"
    import Workflow from "$lib/conn/Workflow.svelte.ts";
    import {goto} from "$app/navigation";
    import {confirm} from "$lib/components/Dialog.svelte";

    let editingFlow = $state();
</script>

<div class="w-full h-full" transition:slide>
    {#if !editingFlow}
        <div class="w-full h-full p-10 flex flex-col gap-5 overflow-y-auto" transition:slide>
            <div class="flex flex-row justify-between items-center">
                <div class="text-3xl font-bold">Your Flows</div>
                <Button onclick={() => {
                if (!userflows.value.flows || !(userflows.value.flows instanceof Array)) {userflows.value.flows = []; userflows.set = true;}
                userflows.value.flows.push({
                    tasks: [],
                    name: 'New Flow',
                    lastUpdated: new Date(),
                    draft: true
                })
                editingFlow = userflows.value.flows.at(-1);
            }}>+ New Flow</Button>
            </div>
            <div class="flex flex-col flex-wrap gap-5">
                {#each (userflows.value.flows || []) as flow, i (flow)}
                    <div class=" bg-neutral-800 w-72 h-72 p-5 flex flex-col justify-between">
                        <div class="text-2xl font-bold line-clamp-4">
                            {flow.name}
                        </div>
                        <div class="flex flex-col gap-2">
                            <div class="text-neutral-300">
                                <div>
                                    {flow.tasks.length} action{flow.tasks.length !== 1 ? 's' : ''}
                                </div>
                                <div>
                                    Last updated on <b>{new Date(flow.lastUpdated).toLocaleDateString()}</b>
                                </div>
                            </div>
                            <div class="flex flex-row justify-end gap-2">
                                <Button onclick={() => {
                                    currentFlow.value = new Workflow(flow.tasks, flow.name)
                                    currentFlow.value.start();
                                    goto("/flow")
                                }}>Run</Button>
                                <Button onclick={() => editingFlow = userflows.value.flows[i]}>Edit</Button>
                                <Button destructive onclick={async () => {
                                    let [result] = await confirm("Delete Flow", "Are you sure you would like to delete this flow? This action cannot be undone.")
                                    if (result) {
                                        userflows.value.flows.splice(i, 1);
                                        userflows.set = true;
                                        await save(userflows);
                                    }
                                }}>
                                    <img src={trash} alt="delete" class="h-6">
                                </Button>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {:else}
        <EditingFlow bind:editingFlow />
    {/if}
</div>