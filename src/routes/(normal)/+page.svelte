<script lang="ts">
    import Spinner from "$lib/components/generic/Spinner.svelte";
    import {app, auth, caddy, error, loadAll} from "$lib/state/states.svelte";
    import {onMount, tick} from "svelte";
    import { load } from '@tauri-apps/plugin-store';
    import {goto} from "$app/navigation";
    import {fade} from "svelte/transition";
    import {invoke} from "@tauri-apps/api/core";
    import image from "$lib/assets/favicon.png";

    onMount(async () => {
        try {
            await loadAll();
            app.value.persistentStoresLoaded = true;

            if (!auth.value?.username) {
                await tick();
                setTimeout(() => {
                    goto("/onboarding");
                }, 2000)
                return;
            }
            try {
                await goto("/main")
            } catch (e) {
                console.error("error going to main, may be due to lack of data", e)
                await goto('/flow')
            }
        } catch (e) {
            console.error("Failed to load data:", e);
            error.value = "Failed to load application data. Please check your configuration.";
            error.set = true;
        }
    })
</script>

<div class="backdrop-blur-md fixed -z-5 top-0 w-full h-full bg-splash-screen flex flex-col items-center justify-center" transition:fade>
    <div class="flex flex-row items-center">
        <img src={image} class="h-20 w-20 mr-4" alt=""/>
        <div class="text-6xl font-sans">NestHelper</div>
    </div>
    <div class="flex flex-row items-center mt-4">
        {#if error.set}
            <div class="flex flex-row items-center max-w-2xl">
                <div class="text-2xl font-sans">{error.value}</div>
                <button class="ml-4 px-4 py-2 bg-red-500 text-white rounded" onclick={() => error.set = false}>
                    Dismiss
                </button>
            </div>
        {:else}
            <Spinner class="mr-2" type="secondary" />
            <div class="text-2xl font-sans">Initialising...</div>
        {/if}
    </div>
</div>