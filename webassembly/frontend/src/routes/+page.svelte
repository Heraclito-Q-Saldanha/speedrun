<script lang="ts">
    import { onMount } from "svelte";
    import init, { sha256 } from "webassembly";

    let input = $state("");
    let output = $state("");

    function hash() {
        output = sha256(input);
    }

    onMount(async () => {
        await init();
    });
</script>

<div class="flex flex-col w-screen h-screen">
    <h1>Sha256</h1>
    <input bind:value={input} />
    <button onclick={hash}>hash</button>
    <p>{output}</p>
</div>
