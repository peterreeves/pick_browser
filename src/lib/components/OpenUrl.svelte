<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import Copy from "@lucide/svelte/icons/copy";
    import Check from "@lucide/svelte/icons/check";
    import Link from "@lucide/svelte/icons/link";
    import BrowserList from "./BrowserList.svelte";
    import type { Browser, Rule } from "$lib/components/defs";
    import { globMatches } from "$lib/glob";

    const PROMPT_TO_CHOOSE = "";

    let urlToOpen = $state(await invoke<string>("url_to_open"));
    let copied = $state(false);
    let closeAfterOpen = $state(true);

    // Create rule state
    let includePath = $state(false);
    let includeQuery = $state(false);
    let ruleBrowserId = $state("");
    let ruleError = $state<string | null>(null);
    let submitting = $state(false);

    // Fetch rules and browsers for the create-rule section
    const rules = $derived(await invoke<Array<Rule>>("get_rules"));
    const browsers = $derived(await invoke<Array<Browser>>("get_browsers"));

    // Parse the URL
    const parsedUrl = $derived.by(() => {
        if (!urlToOpen) return null;
        try {
            const url = new URL(urlToOpen);
            return {
                origin: url.origin,
                pathname: url.pathname,
                search: url.search,
            };
        } catch {
            return null;
        }
    });

    // Build the pattern preview
    const rulePattern = $derived.by(() => {
        if (!parsedUrl) return "";
        let pattern = parsedUrl.origin;
        if (includePath) {
            pattern += parsedUrl.pathname;
        }
        if (includeQuery && includePath) {
            pattern += parsedUrl.search;
        }
        pattern += "*";
        return pattern;
    });

    // Check if URL already matches an existing rule
    const matchingRule = $derived.by(() => {
        if (!urlToOpen) return null;
        for (const rule of rules) {
            if (globMatches(rule.pattern, urlToOpen)) {
                return rule;
            }
        }
        return null;
    });

    const matchingBrowserName = $derived.by(() => {
        if (!matchingRule) return "";
        if (matchingRule.browser_id === PROMPT_TO_CHOOSE) return "Prompt to choose";
        const browser = browsers.find((b) => b.id === matchingRule.browser_id);
        return browser?.name ?? "Unknown";
    });

    // TODO: Replace this $effect with an onchange handler on the includePath checkbox
    // Reset includeQuery when includePath is unchecked
    $effect(() => {
        if (!includePath) {
            includeQuery = false;
        }
    });

    const checkAndAutoOpen = async (url: string) => {
        if (!url) return;
        const browserId = await invoke<string | null>("check_rules", { url });
        if (browserId) {
            await invoke("open_url_in_browser", { url, id: browserId, close: closeAfterOpen });
        }
    };

    onMount(() => {
        checkAndAutoOpen(urlToOpen);

        const unlisten = listen<string>("url-opened", (event) => {
            urlToOpen = event.payload;
            checkAndAutoOpen(event.payload);
        });
        return () => {
            unlisten.then((fn) => fn());
        };
    });

    const copyUrl = async () => {
        await window.navigator.clipboard.writeText(urlToOpen);
        copied = true;
        setTimeout(() => {
            copied = false;
        }, 2000);
    };

    const submitRule = async () => {
        if (!rulePattern) return;

        submitting = true;
        ruleError = null;

        try {
            await invoke("add_rule", {
                pattern: rulePattern,
                browserId: ruleBrowserId,
            });
            window.location.reload();
        } catch (err) {
            ruleError = String(err);
        } finally {
            submitting = false;
        }
    };
</script>

{#snippet highlightPattern(pattern: string)}
    {@const parts = pattern.split("*")}
    {#each parts as part, i (i)}
        {part}{#if i < parts.length - 1}<span class="wildcard">*</span>{/if}
    {/each}
{/snippet}

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

    {#if urlToOpen}
        <details class="create-rule-details">
            <summary>Create rule from this URL</summary>
            <div class="create-rule-body">
                {#if matchingRule}
                    <div class="rule-match-notice">
                        This URL already matches rule: {matchingRule.pattern} &rarr; {matchingBrowserName}
                    </div>
                {/if}

                {#if parsedUrl}
                    <div class="create-rule-options">
                        <label class="create-rule-option">
                            <input type="checkbox" bind:checked={includePath} />
                            <span>Include path ({parsedUrl.pathname})</span>
                        </label>
                        <label class="create-rule-option">
                            <input
                                type="checkbox"
                                bind:checked={includeQuery}
                                disabled={!includePath}
                            />
                            <span>Include query ({parsedUrl.search || "none"})</span>
                        </label>
                    </div>

                    <div class="pattern-preview">
                        {@render highlightPattern(rulePattern)}
                    </div>
                {/if}

                <div class="create-rule-row">
                    <select bind:value={ruleBrowserId}>
                        <option value={PROMPT_TO_CHOOSE}>Prompt to choose</option>
                        {#each browsers as browser (browser.id)}
                            <option value={browser.id}>{browser.name}</option>
                        {/each}
                    </select>
                    <button class="btn" onclick={submitRule} disabled={submitting || !rulePattern}>
                        {#if submitting}
                            Adding...
                        {:else}
                            Add Rule
                        {/if}
                    </button>
                </div>

                {#if ruleError}
                    <p class="create-rule-error">{ruleError}</p>
                {/if}
            </div>
        </details>
    {/if}

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

    .create-rule-details {
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        background-color: var(--bg-secondary);
        font-size: 0.875rem;
    }

    .create-rule-details summary {
        padding: 0.625rem 0.75rem;
        cursor: pointer;
        color: var(--text-secondary);
        font-weight: 500;
        user-select: none;
    }

    .create-rule-details summary:hover {
        color: var(--text-primary);
    }

    .create-rule-details[open] summary {
        border-bottom: 1px solid var(--border-color);
    }

    .create-rule-body {
        padding: 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .rule-match-notice {
        font-size: 0.8125rem;
        color: var(--text-muted);
        padding: 0.5rem 0.625rem;
        background-color: var(--bg-primary);
        border-radius: 0.375rem;
        border: 1px solid var(--border-color);
    }

    .pattern-preview {
        font-family: monospace;
        font-size: 0.875rem;
        color: var(--text-primary);
        padding: 0.5rem 0.625rem;
        background-color: var(--bg-primary);
        border-radius: 0.375rem;
        border: 1px solid var(--border-color);
    }

    .wildcard {
        color: var(--accent);
        font-weight: 700;
    }

    .create-rule-options {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .create-rule-option {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.8125rem;
        color: var(--text-secondary);
        cursor: pointer;
    }

    .create-rule-option input[type="checkbox"] {
        width: 0.875rem;
        height: 0.875rem;
        accent-color: var(--accent);
        cursor: pointer;
    }

    .create-rule-option input[type="checkbox"]:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .create-rule-row {
        display: flex;
        gap: 0.5rem;
        align-items: stretch;
    }

    .create-rule-row select {
        flex: 1;
        font-size: 0.8125rem;
    }

    .create-rule-row .btn {
        flex-shrink: 0;
        font-size: 0.8125rem;
    }

    .create-rule-error {
        font-size: 0.8125rem;
        color: var(--danger-text);
    }
</style>
