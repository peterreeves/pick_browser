<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Copy from "@lucide/svelte/icons/copy";
    import Check from "@lucide/svelte/icons/check";
    import Link from "@lucide/svelte/icons/link";
    import BrowserList from "./BrowserList.svelte";

    let urlToOpen = $derived(await invoke<string>("url_to_open"));
    let copied = $state(false);
    let closeAfterOpen = $state(true);

    const copyUrl = async () => {
        await window.navigator.clipboard.writeText(urlToOpen);
        copied = true;
        setTimeout(() => {
            copied = false;
        }, 2000);
    };
</script>

<section class="open-url">
    <div class="url-container">
        <div class="url-input-wrapper">
            <Link size={16} class="url-icon" />
            <input
                bind:value={urlToOpen}
                id="url-to-open"
                type="url"
                placeholder="No URL provided"
                autocomplete="off"
            />
        </div>
        <button class="btn-icon copy-btn" onclick={copyUrl} aria-label="Copy URL">
            {#if copied}
                <Check size={18} />
            {:else}
                <Copy size={18} />
            {/if}
        </button>
    </div>

    <label class="close-after-open">
        <input type="checkbox" bind:checked={closeAfterOpen} />
        <span>Close after opening browser</span>
    </label>

    <BrowserList {urlToOpen} {closeAfterOpen} />
</section>

<style>
    .open-url {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    .url-container {
        display: flex;
        gap: 0.5rem;
        align-items: stretch;
    }

    .url-input-wrapper {
        flex: 1;
        position: relative;
        display: flex;
        align-items: center;
    }

    .url-input-wrapper :global(.url-icon) {
        position: absolute;
        left: 0.75rem;
        color: var(--text-muted);
        pointer-events: none;
    }

    #url-to-open {
        width: 100%;
        padding-left: 2.25rem;
        font-size: 0.875rem;
    }

    .copy-btn {
        flex-shrink: 0;
    }

    .close-after-open {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.875rem;
        color: var(--text-secondary);
        cursor: pointer;
    }

    .close-after-open input[type="checkbox"] {
        width: 1rem;
        height: 1rem;
        accent-color: var(--accent);
        cursor: pointer;
    }
</style>
