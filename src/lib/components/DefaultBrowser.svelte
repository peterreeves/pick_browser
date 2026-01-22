<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import CircleCheck from "@lucide/svelte/icons/circle-check";
    import CircleX from "@lucide/svelte/icons/circle-x";
    import Loader2 from "@lucide/svelte/icons/loader-2";

    let isDefault = $derived(await invoke<boolean>("is_default_browser"));
    let makingDefault = $state(false);

    const makeDefault = async () => {
        try {
            makingDefault = true;
            await invoke<void>("make_default_browser");
            isDefault = true;
        } finally {
            makingDefault = false;
        }
    };
</script>

<div class="status">
    {#if isDefault}
        <span class="badge badge-success">
            <CircleCheck size={14} />
            <span>Default Browser</span>
        </span>
    {:else}
        <button class="badge badge-warning" onclick={makeDefault} disabled={makingDefault}>
            {#if makingDefault}
                <Loader2 size={14} class="spinner" />
            {:else}
                <CircleX size={14} />
            {/if}
            <span>Set as Default</span>
        </button>
    {/if}
</div>

<style>
    .status {
        display: flex;
        align-items: center;
    }

    .badge {
        display: inline-flex;
        align-items: center;
        gap: 0.375rem;
        padding: 0.25rem 0.625rem;
        border-radius: 9999px;
        font-size: 0.75rem;
        font-weight: 500;
        border: none;
    }

    .badge-success {
        background-color: var(--green-100);
        color: var(--green-700);
    }

    .badge-warning {
        background-color: var(--yellow-100);
        color: var(--yellow-700);
        cursor: pointer;
        transition: background-color 0.15s ease;
    }

    .badge-warning:hover:not(:disabled) {
        background-color: var(--yellow-200);
    }

    .badge-warning:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }

    :global(.spinner) {
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
</style>
