<script lang="ts">
import Flow from "$lib/components/Flow.svelte";
import {slide} from "svelte/transition";
import {onMount} from "svelte";
import {app, currentFlow} from "$lib/state/states.svelte";
import Button from "$lib/components/Button.svelte";

onMount(() => {
    app.value.status = ""
})
</script>

<div class="w-full min-h-full flex flex-col p-10 bg-purple-900" transition:slide>
    {#if currentFlow.value}
        <div class="font-normal text-4xl mb-5 tracking-tight">
            Flow: <b>{currentFlow.value.name}</b>
        </div>
        <div class="rounded-2xl bg-neutral-900 p-5 flex flex-col gap-2">
            <div>
                Status: <b>{currentFlow.value.complete ? 'Completed' : currentFlow.value.failed ? 'Failed' : currentFlow.value.started ? 'Ongoing' : 'Pending'}</b>
            </div>
            <div>
                Started: <b>{new Date(currentFlow.value.startedAt).toLocaleTimeString()}</b>
            </div>
            {#if currentFlow.value.finishedAt}
                <div>
                    Completed: <b>{new Date(currentFlow.value.finishedAt).toLocaleTimeString()}</b>
                </div>
            {/if}
        </div>
        <div class="p-2">
            <div class="flex flex-row items-center justify-end">
                {#if currentFlow.value.failed}
                    <Button destructive onclick={() => currentFlow.value.promise.cancel()}>Cancel Flow</Button>
                {/if}
                {#if !(currentFlow.value.complete || currentFlow.value.failed)}
                    <Button destructive onclick={() => currentFlow.value.promise.cancel()}>Cancel Flow</Button>
                {/if}
            </div>
        </div>
        <div class="">
            <Flow />
        </div>
    {/if}
</div>