<script>
    import {onMount, tick} from "svelte";
    import {invoke} from '@tauri-apps/api/core';
    import Spinner from "$lib/components/Spinner.svelte";
    import {goto} from "$app/navigation";
    import Button from "$lib/components/Button.svelte";
    import {auth, currentFlow, services} from "$lib/state/states.svelte.js";
    import Workflow from "$lib/conn/Workflow.svelte.js";
    import Flows from "$lib/conn/Flows.ts";

    console.log(services.value)
</script>

<div class="w-full h-full flex flex-col lg:p-5 xl:p-10 gap-5">
    <div class="text-6xl font-sans">
        Welcome, <b>{auth.value.username || "Guest"}!</b>
    </div>
    <div class="flex flex-row flex-1">
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-center flex-1">
            <Button onclick={() => {currentFlow.value = new Workflow(Flows.startup); currentFlow.value.start()}}>start
                flow (debug)
            </Button>
        </div>
    </div>
    <div class="flex flex-row flex-1">
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-start flex-1">
            <div class="text-sm">Services</div>
            <div class="flex flex-row items-center justify-center gap-2 text-7xl font-bold tracking-wider">
                <span class="relative flex size-8 mx-4 {services.value.filter(s => s.active === 'active').length === services.value.length ? '*:bg-green-500' : services.value.filter(s => s.active === 'active').length > services.value.length / 2 ? '*:bg-yellow-500' : '*:bg-red-500'}">
                    <span class="absolute inline-flex h-full w-full animate-[ping_2s_cubic-bezier(0,0,0.2,1)_infinite] rounded-full opacity-75"></span>
                    <span class="relative inline-flex size-8 rounded-full"></span>
                </span>
                <div>{services.value.filter(s => s.active === "active").length}/{services.value.length}</div>
            </div>
            <div>
                currently active
            </div>
        </div>
    </div>
</div>