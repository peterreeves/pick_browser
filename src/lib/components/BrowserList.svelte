<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import { DropdownMenu } from "bits-ui";
    import Globe from "@lucide/svelte/icons/globe";
    import Plus from "@lucide/svelte/icons/plus";
    import Pencil from "@lucide/svelte/icons/pencil";
    import Trash2 from "@lucide/svelte/icons/trash-2";
    import type { Browser, BrowserIcon } from "./defs";
    import EllipsisVertical from "@lucide/svelte/icons/ellipsis-vertical";

    type Props = {
        urlToOpen?: string;
        closeAfterOpen?: boolean;
    };

    let { urlToOpen, closeAfterOpen = true }: Props = $props();

    let openingBrowser = $state<string | null>(null);

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
            openingBrowser = id;
            await invoke<void>("open_url_in_browser", {
                url: urlToOpen,
                id: id,
                close: closeAfterOpen,
            });
        } finally {
            openingBrowser = null;
        }
    };

    const deleteBrowser = async (id: string) => {
        await invoke<void>("delete_browser", { id });
        window.location.reload();
    };
</script>

<div class="browser-grid">
    {#each browsers as browser (browser.id)}
        {@const browserIcon = await getBrowserIcon(browser)}
        <div class="browser-card">
            <DropdownMenu.Root>
                <DropdownMenu.Trigger class="browser-menu" aria-label="Browser options">
                    <EllipsisVertical></EllipsisVertical>
                </DropdownMenu.Trigger>
                <DropdownMenu.Portal>
                    <DropdownMenu.Content class="dropdown-content" sideOffset={4} align="end">
                        <DropdownMenu.Item
                            class="dropdown-item"
                            onclick={() => goto(`/edit/${browser.id}`)}
                        >
                            <Pencil size={14} />
                            <span>Edit {browser.name}</span>
                        </DropdownMenu.Item>
                        <DropdownMenu.Item
                            class="dropdown-item dropdown-item-danger"
                            onclick={() => deleteBrowser(browser.id)}
                        >
                            <Trash2 size={14} />
                            <span>Delete {browser.name}</span>
                        </DropdownMenu.Item>
                    </DropdownMenu.Content>
                </DropdownMenu.Portal>
            </DropdownMenu.Root>

            <button
                onclick={() => openBrowser(browser.id)}
                class="browser-btn"
                disabled={openingBrowser !== null}
            >
                <div class="browser-icon">
                    {#if browserIcon !== null}
                        <img src={browserIcon} alt="" class="icon-img" />
                    {:else}
                        <Globe size={40} strokeWidth={1.5} />
                    {/if}
                </div>
                <span class="browser-name">{browser.name}</span>
            </button>
        </div>
    {/each}

    <div class="browser-card">
        <a href="/new" class="add-new">
            <div class="browser-icon add-icon">
                <Plus size={32} strokeWidth={1.5} />
            </div>
            <span class="browser-name">Add new</span>
        </a>
    </div>
</div>

<style>
    .browser-grid {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 0.75rem;
    }

    .browser-card {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        border-radius: 0.75rem;
        background-color: var(--bg-secondary);
        border: 1px solid var(--border-color);
        overflow: hidden;
        transition:
            border-color 0.15s ease,
            box-shadow 0.15s ease;
    }

    .browser-card:hover {
        border-color: var(--zinc-400);
        box-shadow: 0 2px 8px rgb(0 0 0 / 0.05);
    }

    :global(.browser-menu) {
        position: absolute;
        top: 4px;
        right: 4px;
        padding: 0px;
        border-radius: 50%;
        width: 24px;
        height: 24px;
        background-color: transparent;
        border: none;
        color: var(--text-muted);
        transition:
            opacity 0.15s ease,
            background-color 0.15s ease;
    }

    :global(.browser-card:hover) :global(.browser-menu) {
        opacity: 1;
    }

    :global(.browser-menu:hover) {
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
    }

    .browser-btn {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
        padding: 8px;
        background: none;
        border: none;
        cursor: pointer;
        width: 100%;
        aspect-ratio: 1;
    }

    .browser-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .browser-icon {
        width: 56px;
        height: 56px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--text-muted);
        /* margin: 0px auto; */
    }

    .icon-img {
        width: 100%;
        height: 100%;
        object-fit: contain;
    }

    .browser-name {
        font-size: 0.8125rem;
        font-weight: 500;
        color: var(--text-primary);
        text-align: center;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        max-width: 100%;
        padding: 0 0.25rem;
    }

    .add-new {
        text-decoration: none;
        cursor: pointer;
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
        padding: 8px;
        aspect-ratio: 1;
    }

    .add-new:hover {
        background-color: var(--bg-tertiary);
    }

    .add-new .browser-icon {
        color: var(--text-muted);
    }

    .add-new .browser-name {
        color: var(--text-secondary);
    }

    /* Inherit dropdown styles from Settings but add danger variant */
    :global(.dropdown-item-danger) {
        color: var(--red-600) !important;
    }

    :global(.dropdown-item-danger:hover),
    :global(.dropdown-item-danger[data-highlighted]) {
        background-color: var(--red-50) !important;
    }
</style>
