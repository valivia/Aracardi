<script lang="ts">
    import type { AddonSummary } from "lib/addon";
    import Tag from "./Tag.svelte";
    import { CardsIcon } from "lib/icons";

    interface Props {
        addon: AddonSummary;
        active: boolean;
        allowNsfw: boolean;
        toggle: (addon: AddonSummary) => void;
    }

    let { addon, active, allowNsfw, toggle }: Props = $props();

    let cardCount = $derived.by(() => {
        let count = addon.cardCount;
        if (!allowNsfw) {
            count -= addon.nsfwCardCount;
        }
        return count;
    });
</script>

<button class="main" class:active onclick={() => toggle(addon)}>
    <h2 class="title">{addon.title}</h2>
    <p class="description">{addon.description}</p>

    <section class="tags">
        <Tag icon={CardsIcon}>
            {cardCount}
        </Tag>
        {#if addon.isOfficial}
            <Tag>Official</Tag>
        {/if}
        {#if addon.isDefault}
            <Tag>Default</Tag>
        {/if}
    </section>
</button>

<style lang="scss">
    @use "styles/abstracts" as *;

    .main {
        border: var(--border-width) solid currentColor;
        border-radius: 1.5rem;
        background-color: transparent;
        color: var(--theme-text);

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 0.4em;

        cursor: pointer;

        font-size: clamp(1rem, 2vw, 1.2rem);
        padding: 0.8rem 1.2rem;

        transform: scale(1);
        transition: transform ease-in-out 100ms;

        @include noselect;

        &:hover,
        &:focus-visible {
            transform: scale(1.02);
            outline: none;
        }

        &:focus-visible {
            outline: var(--outline-focus);
            outline-offset: var(--outline-focus-offset);
        }

        &.active {
            color: var(--theme-accent);

            .title {
                text-decoration: underline;
            }

            &::after {
                content: "";
                width: 6px;
                height: 6px;
                background-color: var(--theme-accent);
                border-radius: 100vw;
                position: absolute;
                right: 1em;
                top: 1em;
            }
        }

        & > .title {
            font-size: 1.2em;
            font-weight: 500;
            margin: 0;

            text-transform: capitalize;
            white-space: nowrap;
            text-overflow: ellipsis;
            overflow: hidden;
        }

        & > .description {
            font-weight: 400;
            font-size: 0.8em;
            margin: 0;
        }

        & > .tags {
            margin-top: auto;
            display: flex;
            gap: 0.5em;
        }
    }
</style>
