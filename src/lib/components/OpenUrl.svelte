<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import BrowserList from "./BrowserList.svelte";

    let urlToOpen = $derived(await invoke<string>("url_to_open"));
</script>

<div>
    <label for="url-to-open">URL:</label>
    <input bind:value={urlToOpen} id="url-to-open" type="url" />
</div>

<BrowserList {urlToOpen}></BrowserList>
