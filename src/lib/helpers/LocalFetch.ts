import { invoke } from '@tauri-apps/api/core';

export async function LocalFetch(
    url: string,
    options: {
        method?: string;
        headers?: { [header: string]: string };
        body?: string;
    } = {}
): Promise<{
    ok: boolean;
    status: number;
    headers: { [header: string]: string };
    text: () => Promise<string>;
    json: () => Promise<any>;
}> {
    const { method, headers, body } = options;

    const result: {
        status: number;
        headers: { [header: string]: string };
        body: string;
    } = await invoke('tauri_fetch', {
        request: {
            url,
            method,
            headers,
            body,
        },
    });

    return {
        ok: result.status >= 200 && result.status <= 299,
        status: result.status,
        headers: result.headers,
        text: async () => result.body,
        json: async () => JSON.parse(result.body),
    };
}
globalThis.LocalFetch = LocalFetch;
