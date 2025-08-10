<script>
import Spinner from "$lib/components/Spinner.svelte";
import {draw, fade} from "svelte/transition";
import {onMount} from "svelte";
import {quintIn} from "svelte/easing";

let data = [
    {
        state: "done",
        task: "logging in",
        output: "emulated output"
    },
    {
        state: "ongoing",
        task: "logging in",
        output: "emulated output"
    },
    {
        state: "inactive",
        task: "logging in",
        output: "emulated output"
    },
    {
        state: "inactive",
        task: "logging in",
        output: "emulated output"
    },
    {
        state: "inactive",
        task: "logging in",
        output: "emulated output"
    },
]

onMount(() => {
    setTimeout(() => {
        data[1].state = "done"
        data[2].state = "ongoing"
    }, 1000)
})
</script>

<div class="flex flex-col items-start justify-center gap-4 p-4">
    {#each data as task}
        <div class="flex flex-row items-center gap-4">
            <div class="*:absolute w-8 h-8">
                {#if task.state === 'inactive'}
                    {@render inactive()}
                {:else if task.state === "ongoing"}
                    {@render empty()}
                {:else if task.state === 'done'}
                    {@render check()}
                {:else if task.state === 'failed'}
                    {@render failed()}
                {/if}
            </div>
            <div>{task.task}</div>
        </div>
    {/each}
</div>
<!--out:fade={{duration: 200}} in:fade={{delay: 200}}-->
{#snippet inactive()}
    <div transition:fade={{easing: quintIn}} class="w-8 h-8 rounded-full border-3 border-blue-500 dark:border-blue-500 animate-pulse flex items-center justify-center transition-colors">
    </div>
{/snippet}

{#snippet empty()}
    <div transition:fade={{easing: quintIn}} class="w-8 h-8 rounded-full  flex items-center justify-center transition-colors">
        <Spinner type="secondary"/>
    </div>
{/snippet}

{#snippet check()}
    <div transition:fade={{easing: quintIn}} class="w-8 h-8 rounded-full bg-purple-500 flex items-center justify-center peer-checked:bg-purple-700 transition-colors">
        <svg
                viewBox="0 0 24 24"
                class="w-5 h-5 text-white scale-100 peer-checked:scale-100"
        >
            <path in:draw
                  fill="none"
                  stroke="currentColor"
                  stroke-width="3"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M5 13l4 4L19 7"
            />
        </svg>
    </div>
{/snippet}

{#snippet failed()}
    <div transition:fade={{easing: quintIn}} class="w-8 h-8 rounded-full bg-red-500 flex items-center justify-center peer-checked:bg-red-700 transition-colors">
        <svg
                viewBox="0 0 24 24"
                class="w-5 h-5 text-white scale-100 peer-checked:scale-100"
        >
            <path in:draw
                  fill="none"
                  stroke="currentColor"
                  stroke-width="3"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M18 6L6 18M6 6l12 12"
            />
        </svg>
    </div>
{/snippet}