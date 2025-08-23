import {load as loadStore} from "@tauri-apps/plugin-store";
import {tick} from "svelte";
import * as exports from "./states.svelte.js";
import Workflow from "$lib/conn/Workflow.svelte.js";
import {type Task} from "$lib/conn/Workflow.svelte";

type State<T> = {
    persistent?: string,
    persistentIgnore?: string[]
    set: boolean,
    value: T | null
}
type WithUpdate<T> = {
    lastUpdated: Date
} & T

type AppReady = {
    persistentStoresLoaded: boolean,
    latestNestData: boolean,
    status: string,
    pageActions: [string, () => (void | Promise<void>)][]
}
export let app = $state<State<AppReady>>({
    set: true,
    value: {
        persistentStoresLoaded: false,
        latestNestData: false,
        status: "",
        pageActions: []
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
    json: {
        admin?: boolean,
        servers?: number
    },
    caddyfile: string // i aint doing visual caddyfile editing yet
}
export let caddy = $state<State<WithUpdate<CaddySettings>>>({
    persistent: "caddy",
    set: false,
    value: null
})

type Flows = {
    tasks: Task[],
    name: string,
    description: string,
    lastUpdated: Date
}
export let userflows = $state<State<Flows[]>>({
    persistent: "userflows",
    set: false,
    value: null
})

type FileFolder = {
    type: "file",
    size: number
} | {
    type: "folder"
    children: { [name: string]: FileFolder }
}

type Filesystem = {
    files: { [name: string]: FileFolder },
    fileData: { [name: string]: {
            original: Uint8Array<ArrayBuffer>,
            modified: Uint8Array<ArrayBuffer>,
            newFile?: boolean,
            deletedFile?: boolean,
        }
    }
    filesWereModified: boolean,
    currentFolder: string[],
}

export let filesystem = $state<State<WithUpdate<Filesystem>>>({
    persistent: "filesystem",
    persistentIgnore: ["fileData", "currentFolder", "filesWereModified"],
    set: false,
    value: null
});

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
type Services = {
    services: Service[]
}
export let services = $state<State<WithUpdate<Services>>>({
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

export async function load(store: any) {
    await tick();
    if (store && typeof store === 'object' && 'set' in store && 'value' in store) {
        if (store.persistent) {
            const storeData = await loadStore(`${store.persistent}.json`);
            if (storeData) {
                store.value = Object.fromEntries(await storeData.entries());
                store.set = true;
            }
        }
    }
}
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
export async function save(state: any) {
    await tick();
    if (state && state.persistent) {
        const storeData = await loadStore(`${state.persistent}.json`);
        for (const [key, value] of Object.entries(state.value || {})) {
            if (!state.persistentIgnore?.includes(key)) await storeData.set(key, value);
        }
        console.log(await storeData.entries())
    }
}
export async function saveAll() {
    for (const state of Object.values(exports) as any) {
        if (state && typeof state === 'object' && 'set' in state && 'value' in state) {
            if (state.persistent) {
                await save(state);
            }
        }
    }
}