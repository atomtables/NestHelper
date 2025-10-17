import {invoke} from "@tauri-apps/api/core";
import {auth} from "$lib/state/states.svelte.js";
import {alert} from "$lib/components/generic/Dialog.svelte"

export const EditFile = async (path: string, data: Uint8Array) => {
    try {
        await invoke("ssh_edit_file", {
            remotePath: path,
            newContent: Array.from(data)
        });
    } catch (e) {
        console.error(e);
        throw e;
    }
}

// export const UploadFile = async (local: string, remote: string) => {
//     try {
//         await invoke("ssh_upload_file", {
//             username: auth.value.username,
//             localPath: local,
//             remotePath: remote
//         });
//     } catch (e) {
//         console.error(e);
//         throw e;
//     }
// }