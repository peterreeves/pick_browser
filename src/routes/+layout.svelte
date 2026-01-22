<script lang="ts">
    import "$lib/styling/reset.css";
    import "$lib/styling/colours.css";

    let { children } = $props();
</script>

{@render children?.()}
