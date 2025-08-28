import {WebviewWindow} from "@tauri-apps/api/webviewWindow";

export async function createWindow(url) {
    return new Promise((res, rej) => {
        const webview = new WebviewWindow('url', {
            url,
            hiddenTitle: true,
            width: 300,
            height: 600,
            x: 1600,
            y: 900
        })
        // since the webview window is created asynchronously,
        // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
        webview.once('tauri://created', function () {
            res()
        })
        webview.once('tauri://error', function (e) {
            console.error("Failed to create window: ", e)
            rej(e)
        })
    })
}