<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import BrowserList from "./BrowserList.svelte";

    let urlToOpen = $derived(await invoke<string>("url_to_open"));

    const copyUrl = async () => {
        await window.navigator.clipboard.writeText(urlToOpen);
    };
</script>

<section class="open-url">
    <div class="url">
        <label for="url-to-open">URL:</label>
        <input bind:value={urlToOpen} id="url-to-open" type="url" />
        <button onclick={copyUrl}>Copy URL</button>
    </div>

    <BrowserList {urlToOpen}></BrowserList>
</section>

<style>
    .open-url {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .url {
        display: flex;
        gap: 4px;
        align-items: baseline;
        width: 100%;
    }

    #url-to-open {
        flex: 1 1 auto;
    }
</style>
