<script lang="ts">
    import type { Component, Snippet } from "svelte";

    interface Props {
        icon?: Component;
        children?: Snippet;
    }

    let { icon: Icon, children }: Props = $props();
</script>

<div class="main" class:icon={Icon}>
    {#if Icon}
        <Icon />
    {/if}
    {#if children}
        <span>{@render children()}</span>
    {/if}
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    .main {
        font-size: clamp(0.75rem, 2vw, 0.9rem);
        display: flex;
        align-items: center;

        border: var(--border-width) solid currentColor;
        border-radius: 100px;

        &.icon > span {
            padding-inline: 0.1em 1em;
        }
        span {
            padding-inline: 1em;
        }
    }

    :global(.main > svg) {
        width: 2em;
        height: 2em;

        padding: 0.4em;
    }
</style>
