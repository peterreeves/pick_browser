<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

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

<div>
    {#if isDefault}
        <p>✅ Default Browser</p>
    {:else}
        <p>❌ Not default Browser</p>
        <button onclick={makeDefault} disabled={makingDefault}>Make Default</button>
    {/if}
</div>
