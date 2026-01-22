<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import ArrowLeft from "@lucide/svelte/icons/arrow-left";
    import Upload from "@lucide/svelte/icons/upload";
    import X from "@lucide/svelte/icons/x";
    import type { Browser, BrowserIcon } from "$lib/components/defs";

    const { params } = $props();

    // Load browser data
    const browserData = $derived(await invoke<Browser>("get_browser", { id: params.id }));

    // Load existing icon if browser has one
    const existingIcon = $derived(
        browserData.icon
            ? await invoke<BrowserIcon | null>("get_browser_icon", { id: params.id })
            : null,
    );

    let iconFile = $state.raw<File | null>(null);
    let iconPreview = $state<string | null>(null);
    let removeIcon = $state(false);
    let submitting = $state(false);

    // Set existing icon preview
    $effect(() => {
        if (existingIcon && !iconPreview && !removeIcon) {
            iconPreview = `data:${existingIcon.mime_type};base64,${existingIcon.data}`;
        }
    });

    const fileToBase64 = (file: File): Promise<string> => {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.addEventListener("load", () => {
                const result = reader.result;
                if (typeof result !== "string") {
                    throw new TypeError(`Typeof result is not 'string', it is '${typeof result}'`);
                }
                const base64 = result.split(",")[1];
                resolve(base64);
            });
            reader.addEventListener("error", reject);
            reader.readAsDataURL(file);
        });
    };

    const handleFileChange = (ev: Event) => {
        const input = ev.target as HTMLInputElement;
        const file = input.files?.[0] ?? null;
        iconFile = file;
        removeIcon = false;

        if (file) {
            const reader = new FileReader();
            reader.addEventListener("load", () => {
                if (typeof reader.result === "string") {
                    iconPreview = reader.result;
                } else {
                    throw new TypeError(
                        `Typeof result is not 'string', it is '${typeof reader.result}'`,
                    );
                }
            });
            reader.readAsDataURL(file);
        } else {
            iconPreview = null;
        }
    };

    const clearIcon = () => {
        iconFile = null;
        iconPreview = null;
        removeIcon = true;
    };

    const saveChanges = async (ev: SubmitEvent) => {
        ev.preventDefault();

        if (browserData.name === "" || browserData.path === "") {
            return;
        }

        submitting = true;

        try {
            let icon: string | null = null;
            let iconMime: string | null = null;

            if (iconFile) {
                icon = await fileToBase64(iconFile);
                iconMime = iconFile.type;
            }

            await invoke<void>("update_browser", {
                id: params.id,
                name: browserData.name,
                path: browserData.path,
                icon: icon,
                iconMime: iconMime,
                removeIcon: removeIcon,
            });

            await goto("/");
        } finally {
            submitting = false;
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

    <h1>Edit Browser</h1>

    <form onsubmit={saveChanges}>
        <div class="form-group">
            <label for="name">Name</label>
            <input
                bind:value={browserData.name}
                id="name"
                type="text"
                placeholder="e.g. Chrome, Firefox"
                required
            />
        </div>

        <div class="form-group">
            <label for="path-to-executable">Path to executable</label>
            <input
                bind:value={browserData.path}
                id="path-to-executable"
                type="text"
                placeholder="e.g. C:\Program Files\..."
                required
            />
        </div>

        <div class="form-group">
            <label for="icon">Icon (optional)</label>
            {#if iconPreview}
                <div class="icon-preview">
                    <img src={iconPreview} alt="Icon preview" />
                    <button
                        type="button"
                        class="btn btn-ghost btn-icon remove-icon"
                        onclick={clearIcon}
                    >
                        <X size={16} />
                    </button>
                </div>
            {:else}
                <label class="file-upload" for="icon">
                    <Upload size={24} />
                    <span>Choose image</span>
                    <span class="file-hint">PNG, JPEG, WebP, or AVIF</span>
                </label>
            {/if}
            <input
                id="icon"
                type="file"
                accept="image/png, image/jpeg, image/webp, image/avif"
                onchange={handleFileChange}
                class="file-input"
            />
        </div>

        <div class="form-actions">
            <a href="/" class="btn">Cancel</a>
            <button
                type="submit"
                class="btn btn-primary"
                disabled={submitting || !browserData.name || !browserData.path}
            >
                {#if submitting}
                    Saving...
                {:else}
                    Save Changes
                {/if}
            </button>
        </div>
    </form>
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

    h1 {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--text-primary);
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

    .form-group input[type="text"] {
        width: 100%;
    }

    .file-input {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border: 0;
    }

    .file-upload {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        padding: 1.5rem;
        border: 2px dashed var(--border-color);
        border-radius: 0.5rem;
        background-color: var(--bg-secondary);
        color: var(--text-secondary);
        cursor: pointer;
        transition:
            border-color 0.15s ease,
            background-color 0.15s ease;
    }

    .file-upload:hover {
        border-color: var(--zinc-400);
        background-color: var(--bg-tertiary);
    }

    .file-upload span {
        font-size: 0.875rem;
        font-weight: 500;
    }

    .file-hint {
        font-size: 0.75rem !important;
        font-weight: 400 !important;
        color: var(--text-muted);
    }

    .icon-preview {
        position: relative;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 80px;
        height: 80px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        background-color: var(--bg-secondary);
        overflow: hidden;
    }

    .icon-preview img {
        width: 100%;
        height: 100%;
        object-fit: contain;
    }

    .remove-icon {
        position: absolute;
        top: 0.25rem;
        right: 0.25rem;
        padding: 0.25rem;
        background-color: var(--bg-primary);
        border-radius: 50%;
    }

    .form-actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        padding-top: 0.5rem;
    }
</style>
