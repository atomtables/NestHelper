import {auth} from "$lib/state/states.svelte.ts";
import {goto} from "$app/navigation";
import {redirect} from "@sveltejs/kit";

export const load = ({ url }) => {
    if (!auth?.value?.username) {
        redirect(301, "/")
    }
}