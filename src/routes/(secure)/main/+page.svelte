<script>
    import {onMount, tick} from "svelte";
    import { invoke } from '@tauri-apps/api/core';
    import Spinner from "$lib/components/Spinner.svelte";
    import {goto} from "$app/navigation";
    import Button from "$lib/components/Button.svelte";

    let verify = new Promise(() => null)
    onMount(async () => {
        // verify = invoke('verifyssh', { username: "atomtables " })
    })
</script>

<h1>dshkjfshjkdjfkhkj</h1>

{#await verify}
    <div class="mt-(--top-bar) fixed backdrop-blur-md z-50 top-0 w-full h-full">
        <div class="w-full h-full flex flex-row justify-center items-center space-x-2">
            <Spinner />
            <div>
                Verifying SSH authentication... this may take a while.
            </div>
            <Button onclick={() => tick().then(() =>goto("/"))}>close</Button>
        </div>
    </div>
{:then verify}
    <h1>Success {verify}</h1>
{:catch verify}
    <div class="mt-(--top-bar) bg-neutral-50/0 fixed backdrop-blur-md z-50 top-0 w-full h-full">
        <div class="w-full h-full flex flex-row justify-center items-center space-x-2">
            <div>
                Failed: {verify}
            </div>
        </div>
    </div>
{/await}