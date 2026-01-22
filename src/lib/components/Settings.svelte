<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { DropdownMenu } from "bits-ui";
    import Settings2 from "@lucide/svelte/icons/settings-2";
    import FileCode from "@lucide/svelte/icons/file-code";
    import RefreshCw from "@lucide/svelte/icons/refresh-cw";

    const openInVscode = async () => {
        await invoke<void>("open_config_in_vscode");
    };

    const reload = async () => {
        window.location.reload();
    };
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger class="btn btn-ghost btn-icon" aria-label="Settings">
        <Settings2 size={20} />
    </DropdownMenu.Trigger>

    <DropdownMenu.Portal>
        <DropdownMenu.Content class="dropdown-content" sideOffset={4} align="end">
            <DropdownMenu.Item class="dropdown-item" onclick={openInVscode}>
                <FileCode size={16} />
                <span>Open Config in VS Code</span>
            </DropdownMenu.Item>
            <DropdownMenu.Item class="dropdown-item" onclick={reload}>
                <RefreshCw size={16} />
                <span>Reload Config</span>
            </DropdownMenu.Item>
        </DropdownMenu.Content>
    </DropdownMenu.Portal>
</DropdownMenu.Root>

<style>
    :global(.dropdown-content) {
        min-width: 200px;
        padding: 0.25rem;
        background-color: var(--bg-primary);
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow:
            0 10px 15px -3px rgb(0 0 0 / 0.1),
            0 4px 6px -4px rgb(0 0 0 / 0.1);
        z-index: 50;
    }

    :global(.dropdown-item) {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 0.75rem;
        border-radius: 0.375rem;
        cursor: pointer;
        font-size: 0.875rem;
        color: var(--text-primary);
        transition: background-color 0.15s ease;
    }

    :global(.dropdown-item:hover),
    :global(.dropdown-item[data-highlighted]) {
        background-color: var(--bg-tertiary);
        outline: none;
    }
</style>
