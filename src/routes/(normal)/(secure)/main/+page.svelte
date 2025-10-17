<script>
    import {onMount} from "svelte";
    import Button from "$lib/components/generic/Button.svelte";
    import {app, auth, caddy, currentFlow, server, services, userflows} from "$lib/state/states.svelte.ts";
    import Workflow from "$lib/conn/Workflow.svelte.ts";
    import Flows from "$lib/conn/Flows.ts";
    import {slide} from "svelte/transition";
    import {wait} from "$lib/components/generic/Dialog.svelte";

    let now = new Date().getTime();
    onMount(() => {
        let x = setInterval(() => {
            now = new Date().getTime();
        }, 1000);

        if (!app.value.latestNestData) {
            currentFlow.value = new Workflow(Flows.startup(), "Startup Data Pull");
            currentFlow.value.start()
            // wait(currentFlow.value.promise, "Pulling latest data from Nest", "This process may take a while, but is necessary for showing the latest data.");
            currentFlow.value.promise.then(() => app.value.latestNestData = true)
        }
        app.value.status = ""
        return () => clearInterval(x);
    })
</script>

<svelte:boundary>
    {#snippet failed(error, reset)}
        <div class="mt-(--top-bar) fixed backdrop-blur-md z-50 top-0 w-full h-full">
            <div class="w-full h-full flex flex-row justify-center items-center space-x-2">
                <Spinner />
                {#if currentFlow.value?.failed}
                    <div>
                        Failed to run startup flow.
                    </div>
                    <Button onclick={() => (currentFlow.value = new Workflow(Flows.startup(), "Startup Data Pull"), currentFlow.value.start(), (currentFlow.value.promise.then(() => currentFlow.value.complete && reset())))}>Restart</Button>
                {:else if currentFlow.value}
                    <div>
                        Loading {currentFlow.value.name}... {currentFlow.value.task}...
                    </div>
                {:else}
                    <div>
                        You need to run the startup flow before continuing.
                    </div>
                    <Button onclick={() => (currentFlow.value = new Workflow(Flows.startup(), "Startup Data Pull"), currentFlow.value.start(), (currentFlow.value.promise.then(() => currentFlow.value.complete && reset())))}>Load flow</Button>
                {/if}
            </div>
        </div>
    {/snippet}

    <div class="w-full h-full flex flex-col lg:p-5 xl:p-10 2xl:max-w-5/6 2xl:mx-auto 2xl:py-32 gap-5" transition:slide>
    <div class="text-6xl font-sans">
        Welcome, <b>{auth.value?.username || "Unknown"}!</b>
    </div>
    <div class="flex flex-row flex-1 gap-5">
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-start flex-1">
            <div class="text-sm">Services</div>
            <div class="flex flex-row items-center justify-center gap-2 text-7xl font-bold tracking-wider">
                <span class="relative flex size-8 mx-4 {services.value?.services.filter(s => s.active === 'active').length === services.value?.services.length ? '*:bg-green-500' : services.value?.services.filter(s => s.active === 'active').length > services.value?.services.length / 2 ? '*:bg-yellow-500' : '*:bg-red-500'}">
                    <span class="absolute inline-flex h-full w-full animate-[ping_2s_cubic-bezier(0,0,0.2,1)_infinite] rounded-full opacity-75"></span>
                    <span class="relative inline-flex size-8 rounded-full"></span>
                </span>
                <div>{services.value?.services.filter(s => s.active === "active").length}/{services.value?.services.length}</div>
            </div>
            <div>
                currently active
            </div>
        </div>
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-start flex-1">
            <div class="text-sm">Caddy Rules</div>
            <div class="flex flex-row items-center justify-center gap-2 text-6xl font-bold">
                <div>{caddy.value?.json.servers} domain{caddy.value?.json.servers !== 1 ? 's' : ''}</div>
            </div>
            <div>
                {#if caddy.value?.json.admin}
                    Admin API is being hosted.
                {:else}
                    Admin API is not being hosted.
                {/if}
            </div>
        </div>
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-start flex-1">
            <div class="text-sm">Your flows</div>
            <div class="flex flex-row items-center justify-center gap-2 text-6xl font-bold">
                <div>{userflows.value?.length || 0} flow{userflows.value?.length !== 1 ? 's' : ''}</div>
            </div>
            <div>
                {#if (userflows.value?.length || 0) === 0}
                    No flows created. Try one out (it's a timesaver).
                {/if}
            </div>
        </div>
<!--        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-center flex-1">-->
<!--            <Button onclick={() => {currentFlow.value = new Workflow(Flows.startup, "Startup Data Pull"); currentFlow.value.start();}}>start-->
<!--                flow (debug)-->
<!--            </Button>-->
<!--        </div>-->
    </div>
    <div class="flex flex-row flex-1 gap-5">
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-start flex-1/2">
            {#if server.value}
                {@const distance = now - server.value.runningSince}
                {@const days = Math.floor(distance / (1000 * 60 * 60 * 24))}
                {@const hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))}
                {@const minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60))}
                {@const seconds = Math.floor((distance % (1000 * 60)) / 1000)}
                <div class="text-sm">Uptime</div>
                <div class="flex flex-col items-start justify-center gap-2 text-7xl font-bold">
                    <div>{days}d {hours}h</div><div>{minutes}m {seconds}s</div>
                </div>
                <div class="flex flex-row items-center gap-2 pt-2">
                    <span class="relative flex size-4 *:bg-green-500">
                        <span class="absolute inline-flex h-full w-full animate-[ping_2s_cubic-bezier(0,0,0.2,1)_infinite] rounded-full opacity-75"></span>
                        <span class="relative inline-flex size-4 rounded-full"></span>
                    </span>
                    {server.value.users} users active
                </div>
            {/if}
        </div>
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-start flex-1/4">
            <div class="text-sm">Disk Usage</div>
            <div class="flex flex-col items-start justify-center gap-2 text-7xl font-bold tracking-tighter">
                <div>{server.value?.diskUsage[0].toFixed(2)}<span class="font-normal">/</span></div>
                <div class="flex flex-row items-center justify-center gap-2">
                    <div>{server.value?.diskUsage[1].toFixed(2)}</div>
                    <div class="flex flex-col h-full text-3xl -space-y-1.5 pt-1">
                        <span>G</span>
                        <span>B</span>
                    </div>
                </div>
            </div>
            <div>
                {Math.floor(server.value?.diskUsage[0]/server.value?.diskUsage[1] * 100)}% used
            </div>
        </div>
        <div class="p-5 rounded-2xl backdrop-blur-2xl bg-neutral-500/25 h-full flex flex-col justify-center items-start flex-1/4">
            <div class="text-sm">Memory Usage</div>
            <div class="flex flex-col items-start justify-center gap-2 text-7xl font-bold tracking-tighter">
                <div>{server.value?.memoryUsage[0].toFixed(2)}<span class="font-normal">/</span></div>
                <div class="flex flex-row items-center justify-center gap-2">
                    <div>{server.value?.memoryUsage[1].toFixed(2)}</div>
                    <div class="flex flex-col h-full text-3xl -space-y-1.5 pt-1">
                        <span>G</span>
                        <span>B</span>
                    </div>
                </div>
            </div>
            <div>
                {Math.floor(server.value?.memoryUsage[0]/server.value?.memoryUsage[1] * 100)}% used
            </div>
        </div>
    </div>
</div>
</svelte:boundary>