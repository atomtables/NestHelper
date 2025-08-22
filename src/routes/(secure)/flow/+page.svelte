<script lang="ts">
import Flow from "$lib/components/Flow.svelte";
import {slide} from "svelte/transition";
import {onMount} from "svelte";
import {app} from "$lib/state/states.svelte";

onMount(() => {
    app.value.status = ""
})
</script>

<div class="w-full h-full" transition:slide>
    <Flow />
</div>