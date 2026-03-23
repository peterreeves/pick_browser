export type Browser = {
    readonly id: string;
    name: string;
    path: string;
    icon: string | null; // File extension if icon exists
};

export type BrowserIcon = {
    data: string; // Base64-encoded image data
    mime_type: string;
};

export type Rule = {
    readonly id: string;
    pattern: string; // Glob pattern to match against URLs (* = wildcard)
    browser_id: string; // ID of the browser to open matching URLs in
};
