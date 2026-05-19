<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        children: Snippet;
        onclick: () => void;
    }

    let { children, onclick }: Props = $props();
</script>

<button {onclick}>
    {@render children?.()}
</button>

<style lang="scss">
    button {
        all: unset;
        text-decoration: underline;

        &:hover {
            cursor: pointer;
            color: var(--theme-accent);
        }
    }
</style>
