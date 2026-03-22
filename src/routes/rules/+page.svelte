<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import ArrowLeft from "@lucide/svelte/icons/arrow-left";
    import Plus from "@lucide/svelte/icons/plus";
    import Pencil from "@lucide/svelte/icons/pencil";
    import Trash2 from "@lucide/svelte/icons/trash-2";
    import Check from "@lucide/svelte/icons/check";
    import X from "@lucide/svelte/icons/x";
    import Search from "@lucide/svelte/icons/search";
    import type { Browser, Rule } from "$lib/components/defs";

    const PROMPT_TO_CHOOSE = "";

    const rules = $derived(await invoke<Array<Rule>>("get_rules"));
    const browsers = $derived(await invoke<Array<Browser>>("get_browsers"));

    // Add rule form state
    let newPattern = $state("");
    let newBrowserId = $state("");
    let addError = $state<string | null>(null);
    let adding = $state(false);

    // Edit state
    let editingId = $state<string | null>(null);
    let editPattern = $state("");
    let editBrowserId = $state("");
    let editError = $state<string | null>(null);
    let saving = $state(false);

    const getBrowserName = (browserId: string): string => {
        if (browserId === PROMPT_TO_CHOOSE) return "Prompt to choose";
        const browser = browsers.find((b) => b.id === browserId);
        return browser?.name ?? "Unknown";
    };

    const startEdit = (rule: Rule) => {
        editingId = rule.id;
        editPattern = rule.pattern;
        editBrowserId = rule.browser_id;
        editError = null;
    };

    const cancelEdit = () => {
        editingId = null;
        editPattern = "";
        editBrowserId = "";
        editError = null;
    };

    const saveEdit = async () => {
        if (!editingId || !editPattern) return;

        saving = true;
        editError = null;

        try {
            await invoke("update_rule", {
                id: editingId,
                pattern: editPattern,
                browserId: editBrowserId,
            });
            window.location.reload();
        } catch (err) {
            editError = String(err);
        } finally {
            saving = false;
        }
    };

    const deleteRule = async (id: string) => {
        try {
            await invoke("delete_rule", { id });
            window.location.reload();
        } catch (err) {
            // silently fail
        }
    };

    // Test URL state
    let testUrl = $state("");
    let testResult = $state<string | null>(null);
    let testing = $state(false);

    const testUrlMatch = async () => {
        if (!testUrl) return;
        testing = true;
        try {
            const browserId = await invoke<string | null>("check_rules", { url: testUrl });
            if (browserId) {
                testResult = `Opens in: ${getBrowserName(browserId)}`;
            } else {
                testResult = "No matching rule — will prompt to choose";
            }
        } catch (err) {
            testResult = `Error: ${String(err)}`;
        } finally {
            testing = false;
        }
    };

    const addRule = async (ev: SubmitEvent) => {
        ev.preventDefault();

        if (!newPattern) return;

        adding = true;
        addError = null;

        try {
            await invoke("add_rule", {
                pattern: newPattern,
                browserId: newBrowserId,
            });
            window.location.reload();
        } catch (err) {
            addError = String(err);
        } finally {
            adding = false;
        }
    };
</script>

<main class="container">
    <header>
        <a href="/" class="back-link">
            <ArrowLeft size={20} />
            <span>Back</span>
        </a>
    </header>

    <div class="title-section">
        <h1>URL Rules</h1>
        <p class="subtitle">Automatically open URLs matching a pattern in a specific browser.</p>
    </div>

    <section class="rules-list">
        {#if rules.length === 0}
            <p class="empty-state">No rules configured yet. Add one below.</p>
        {:else}
            {#each rules as rule (rule.id)}
                {#if editingId === rule.id}
                    <div class="rule-item editing">
                        <div class="edit-fields">
                            <div class="form-group">
                                <label for="edit-pattern">Pattern</label>
                                <input
                                    bind:value={editPattern}
                                    id="edit-pattern"
                                    type="text"
                                    placeholder="e.g. github\.com"
                                    class="mono-input"
                                />
                            </div>
                            <div class="form-group">
                                <label for="edit-browser">Browser</label>
                                <select bind:value={editBrowserId} id="edit-browser">
                                    <option value={PROMPT_TO_CHOOSE}>Prompt to choose</option>
                                    {#each browsers as browser (browser.id)}
                                        <option value={browser.id}>{browser.name}</option>
                                    {/each}
                                </select>
                            </div>
                            {#if editError}
                                <p class="error-message">{editError}</p>
                            {/if}
                        </div>
                        <div class="rule-actions">
                            <button
                                class="btn btn-ghost btn-icon"
                                onclick={saveEdit}
                                disabled={saving || !editPattern}
                                aria-label="Save"
                            >
                                <Check size={16} />
                            </button>
                            <button
                                class="btn btn-ghost btn-icon"
                                onclick={cancelEdit}
                                aria-label="Cancel"
                            >
                                <X size={16} />
                            </button>
                        </div>
                    </div>
                {:else}
                    <div class="rule-item">
                        <div class="rule-info">
                            <code class="rule-pattern">{rule.pattern}</code>
                            <span class="rule-browser">{getBrowserName(rule.browser_id)}</span>
                        </div>
                        <div class="rule-actions">
                            <button
                                class="btn btn-ghost btn-icon"
                                onclick={() => startEdit(rule)}
                                aria-label="Edit rule"
                            >
                                <Pencil size={16} />
                            </button>
                            <button
                                class="btn btn-ghost btn-icon btn-danger"
                                onclick={() => deleteRule(rule.id)}
                                aria-label="Delete rule"
                            >
                                <Trash2 size={16} />
                            </button>
                        </div>
                    </div>
                {/if}
            {/each}
        {/if}
    </section>

    <section class="test-section">
        <h2>Test URL</h2>
        <div class="test-input-row">
            <div class="url-input-wrapper">
                <Search size={16} class="search-icon" />
                <input
                    bind:value={testUrl}
                    type="text"
                    placeholder="Paste a URL to test..."
                    class="test-input"
                    autocomplete="off"
                    autocapitalize="off"
                    autocorrect="off"
                    spellcheck="false"
                    onkeydown={(e) => e.key === "Enter" && testUrlMatch()}
                />
            </div>
            <button
                class="btn"
                onclick={testUrlMatch}
                disabled={testing || !testUrl}
            >
                Test
            </button>
        </div>
        {#if testResult}
            <p class="test-result">{testResult}</p>
        {/if}
    </section>

    <section class="add-section">
        <h2>Add Rule</h2>
        <form onsubmit={addRule}>
            <div class="form-group">
                <label for="new-pattern">Pattern</label>
                <input
                    bind:value={newPattern}
                    id="new-pattern"
                    type="text"
                    placeholder="e.g. github\.com"
                    class="mono-input"
                />
            </div>

            <div class="form-group">
                <label for="new-browser">Browser</label>
                <select bind:value={newBrowserId} id="new-browser">
                    <option value={PROMPT_TO_CHOOSE}>Prompt to choose</option>
                    {#each browsers as browser (browser.id)}
                        <option value={browser.id}>{browser.name}</option>
                    {/each}
                </select>
            </div>

            {#if addError}
                <p class="error-message">{addError}</p>
            {/if}

            <div class="form-actions">
                <button
                    type="submit"
                    class="btn btn-primary"
                    disabled={adding || !newPattern}
                >
                    <Plus size={16} />
                    {#if adding}
                        Adding...
                    {:else}
                        Add Rule
                    {/if}
                </button>
            </div>
        </form>
    </section>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        padding: 1.5rem;
        max-width: 480px;
        margin: 0 auto;
    }

    header {
        display: flex;
        align-items: center;
    }

    .back-link {
        display: inline-flex;
        align-items: center;
        gap: 0.375rem;
        color: var(--text-secondary);
        text-decoration: none;
        font-size: 0.875rem;
        font-weight: 500;
        transition: color 0.15s ease;
    }

    .back-link:hover {
        color: var(--text-primary);
        text-decoration: none;
    }

    .title-section {
        display: flex;
        flex-direction: column;
        gap: 0.375rem;
    }

    h1 {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--text-primary);
    }

    .subtitle {
        font-size: 0.875rem;
        color: var(--text-muted);
    }

    h2 {
        font-size: 1.125rem;
        font-weight: 600;
        color: var(--text-primary);
        margin-bottom: 0.75rem;
    }

    .rules-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .empty-state {
        font-size: 0.875rem;
        color: var(--text-muted);
        text-align: center;
        padding: 1.5rem;
        border: 1px dashed var(--border-color);
        border-radius: 0.5rem;
    }

    .rule-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.75rem;
        padding: 0.75rem;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        background-color: var(--bg-secondary);
    }

    .rule-item.editing {
        flex-direction: column;
        align-items: stretch;
    }

    .rule-info {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        min-width: 0;
    }

    .rule-pattern {
        font-family: monospace;
        font-size: 0.875rem;
        color: var(--text-primary);
        word-break: break-all;
    }

    .rule-browser {
        font-size: 0.75rem;
        color: var(--text-muted);
    }

    .rule-actions {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        flex-shrink: 0;
    }

    .editing .rule-actions {
        justify-content: flex-end;
    }

    .edit-fields {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .btn-danger {
        color: var(--danger-text);
    }

    .btn-danger:hover {
        background-color: var(--danger-bg-hover);
    }

    .test-section {
        border-top: 1px solid var(--border-color);
        padding-top: 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .test-input-row {
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

    .url-input-wrapper :global(.search-icon) {
        position: absolute;
        left: 0.75rem;
        color: var(--text-muted);
        pointer-events: none;
    }

    .test-input {
        width: 100%;
        padding: 0.5rem 0.75rem 0.5rem 2.25rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border-color);
        background-color: var(--bg-primary);
        color: var(--text-primary);
        font-size: 0.875rem;
    }

    .test-input:focus {
        outline: none;
        border-color: var(--accent);
        box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent) 20%, transparent);
    }

    .test-result {
        font-size: 0.8125rem;
        color: var(--text-secondary);
        padding: 0.5rem 0.75rem;
        background-color: var(--bg-secondary);
        border-radius: 0.375rem;
        border: 1px solid var(--border-color);
    }

    .add-section {
        border-top: 1px solid var(--border-color);
        padding-top: 1.5rem;
    }

    form {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .form-group label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--text-primary);
    }

    .form-group input[type="text"],
    .form-group select {
        width: 100%;
    }

    .mono-input {
        font-family: monospace;
    }

    .error-message {
        font-size: 0.8125rem;
        color: var(--danger-text);
    }

    .form-actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        padding-top: 0.5rem;
    }

    .form-actions .btn-primary {
        display: inline-flex;
        align-items: center;
        gap: 0.375rem;
    }
</style>
