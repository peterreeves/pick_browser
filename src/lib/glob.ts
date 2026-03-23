/** Convert a glob pattern (where `*` matches anything) into a RegExp. */
export function globToRegex(pattern: string): RegExp {
    const escaped = pattern
        .split("*")
        .map((s) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"))
        .join(".*");
    return new RegExp(`^${escaped}$`, "i");
}

/** Test whether a URL matches a glob pattern. */
export function globMatches(pattern: string, url: string): boolean {
    return globToRegex(pattern).test(url);
}
