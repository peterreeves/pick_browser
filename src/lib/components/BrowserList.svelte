<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { SvelteMap } from "svelte/reactivity";
    import type { Browser, BrowserIcon } from "./defs";

    type Props = {
        urlToOpen?: string;
    };

    let { urlToOpen }: Props = $props();

    let openingBrowser = $state(false);
    let iconCache = new SvelteMap<string, string>();

    let browsers = $derived(await invoke<Array<Browser>>("get_browsers"));

    const loadIcon = async (browser: Browser) => {
        if (!browser.icon) return;
        if (iconCache.has(browser.id)) return;

        const icon = await invoke<BrowserIcon | null>("get_browser_icon", { id: browser.id });
        if (icon) {
            iconCache.set(browser.id, `data:${icon.mime_type};base64,${icon.data}`);
        }
    };

    $effect(() => {
        browsers.forEach((browser) => loadIcon(browser));
    });

    const openBrowser = async (id: string) => {
        try {
            openingBrowser = true;
            await invoke<void>("open_url_in_browser", { url: urlToOpen, id: id });
        } finally {
            openingBrowser = false;
        }
    };
</script>

<div class="browser-grid">
    {#each browsers as browser (browser.id)}
        <div class="browser">
            <button>
                <!-- TODO: Add menu button to allow editing/removing this entry -->
            </button>
            <button
                onclick={() => openBrowser(browser.id)}
                class="browser-icon"
                disabled={openingBrowser}
            >
                {#if iconCache.has(browser.id)}
                    <img src={iconCache.get(browser.id)} alt={browser.name} class="icon-img" />
                {:else}
                    🌐
                {/if}
            </button>
            <p>{browser.name}</p>
        </div>
    {/each}
    <div class="browser">
        <a href="/new" class="browser-icon">➕</a>
        <p>Add new</p>
    </div>
</div>

<style>
    .browser-grid {
        display: grid;
        grid-template-columns: repeat(4, 128px);
        gap: 8px;
        border-radius: 8px;
    }

    .browser {
        border: 1px solid var(--zinc-500);
        padding: 4px;
        aspect-ratio: 1;
    }

    .browser-icon {
        font-size: 2.5rem;
        line-height: 1;
        text-decoration: none;
    }

    .icon-img {
        width: 2.5rem;
        height: 2.5rem;
        object-fit: contain;
    }
</style>
