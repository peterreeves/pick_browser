<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Browser, BrowserIcon } from "./defs";

    type Props = {
        urlToOpen?: string;
    };

    let { urlToOpen }: Props = $props();

    let openingBrowser = $state(false);

    let browsers = $derived(await invoke<Array<Browser>>("get_browsers"));

    const getBrowserIcon = async (browser: Browser): Promise<string | null> => {
        if (browser.icon === null) {
            return null;
        }

        try {
            const icon = await invoke<BrowserIcon | null>("get_browser_icon", { id: browser.id });
            if (icon !== null) {
                return `data:${icon.mime_type};base64,${icon.data}`;
            } else {
                return null;
            }
        } catch {
            return null;
        }
    };

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
        {@const browserIcon = await getBrowserIcon(browser)}
        <div class="browser">
            <button class="browser-menu">
                <!-- TODO: Add menu button to allow editing/removing this entry -->
            </button>
            <button
                onclick={() => openBrowser(browser.id)}
                class="browser-icon"
                disabled={openingBrowser}
            >
                {#if browserIcon !== null}
                    <img src={browserIcon} alt={browser.name} class="icon-img" />
                {:else}
                    🌐
                {/if}
                <p class="browser-name">{browser.name}</p>
            </button>
        </div>
    {/each}
    <div class="browser">
        <div class="browser-new">
            <a href="/new">➕</a>
            <p>Add new</p>
        </div>
    </div>
</div>

<style>
    .browser-grid {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr;
        gap: 8px;
        border-radius: 8px;
    }

    .browser {
        aspect-ratio: 1;
        position: relative;
        display: flex;
        flex-direction: column;
        width: 128px;
        height: 128px;
        overflow: hidden;
    }

    .browser-menu {
        position: absolute;
        top: 8px;
        right: 8px;
        width: 24px;
        aspect-ratio: 1;
        border-radius: 50%;
    }

    .browser-icon,
    .browser-new {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-decoration: none;
        padding: 0px;
        background-color: var(--zinc-200);
        border: 1px solid var(--zinc-900);
        border-radius: 16px;
        overflow: hidden;
    }

    .icon-img {
        object-fit: contain;
        flex: 1 1 auto;
        height: 0px;
    }
</style>
