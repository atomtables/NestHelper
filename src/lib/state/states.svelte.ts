import {load as loadStore} from "@tauri-apps/plugin-store";
import {tick} from "svelte";
import * as exports from "./states.svelte.js";

type State<T> = {
    persistent?: string,
    set: boolean,
    value: T | null
}

export let appReady = $state<State<Boolean>>({
    set: true,
    value: false
});

type Authentication = {
    username: string
}
export let auth = $state<State<Authentication>>({
    persistent: "auth",
    set: false,
    value: null
});
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