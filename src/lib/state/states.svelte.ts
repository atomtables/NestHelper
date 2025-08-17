import {load as loadStore} from "@tauri-apps/plugin-store";
import {tick} from "svelte";
import * as exports from "./states.svelte.js";
import Workflow from "$lib/conn/Workflow.svelte.js";

type State<T> = {
    persistent?: string,
    set: boolean,
    value: T | null
}
type WithUpdate<T> = {
    lastUpdated: Date
} & T

type AppReady = {
    persistentStoresLoaded: boolean,
    latestNestData: boolean
}
export let appReady = $state<State<AppReady>>({
    set: true,
    value: {
        persistentStoresLoaded: false,
        latestNestData: false
    }
});

// Persistent
type Authentication = {
    username: string
}
export let auth = $state<State<Authentication>>({
    persistent: "auth",
    set: false,
    value: null
});

// Persistent
type CaddySettings = {
    domains: [string, string][],
    caddyfile: string // i aint doing visual caddyfile editing yet
}
export let caddy = $state<State<WithUpdate<CaddySettings>>>({
    persistent: "caddy",
    set: false,
    value: null
})

// Persistent
type Service = {
    name: string,
    description: string,
    loaded: "loaded",
    path: string,
    enabled: "enabled" | "disabled",
    preset: "enabled" | "disabled",
    active: "active" | "inactive" | "failed",
    status: string, // if failed this is the result
    since: Date,
    pid: number,
    exec: string // if failed this is the exit code
}
export let services = $state<State<WithUpdate<Service[]>>>({
    persistent: "services",
    set: false,
    value: null
})

// Persistent
type NestServer = {
    diskUsage: [number, number],
    memoryUsage: [number, number],
    users: number,
    runningSince: Date
}
export let server = $state<State<WithUpdate<NestServer>>>({
    persistent: "server",
    set: false,
    value: null
})

export let currentFlow = $state<State<typeof Workflow>>({
    set: false,
    value: null
})

export let error = $state<State<string>>({
    set: false,
    value: null
})

export async function loadAll() {
    for (const state of Object.values(exports) as any) {
        if (state && typeof state === 'object' && 'set' in state && 'value' in state) {
            if (state.persistent) {
                const store = await loadStore(`${state.persistent}.json`);
                if (store) {
                    state.value = Object.fromEntries(await store.entries())
                    state.set = true;
                }
            }
        }
    }
}
export async function load(store: string) {
    const state = Array(exports).find((s: any) => typeof s === 'object' && 'persistent' in state && s.persistent === store);
    if (state && state.persistent) {
        const loaded = await loadStore(`${state.persistent}.json`);
        if (loaded) {
            state.value = Object.fromEntries(await loaded.entries());
            state.set = true;
        }
    }
    await tick();
}
export async function save(state: any) {
    await tick();
    if (state && state.persistent) {
        const storeData = await loadStore(`${state.persistent}.json`);
        if (storeData) {
            for (const [key, value] of Object.entries(state.value || {})) {
                console.log(key, value)
                await storeData.set(key, value);
            }
        }
        console.log(await storeData.entries())
    }
}
export function arrWithUpdate<T>(arr: T[]): WithUpdate<T[]> {
    let newArr = arr as WithUpdate<T[]>
    newArr.lastUpdated = new Date();
    return newArr;
}