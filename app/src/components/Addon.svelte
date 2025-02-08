<script lang="ts">
    import type { AddonSummary } from "lib/addon";
    import Tag from "./Tag.svelte";
    import { CardsIcon } from "lib/icons";

    interface Props {
        addon: AddonSummary;
        active: boolean;
        toggle: (addon: AddonSummary) => void;
    }

    let { addon, active, toggle }: Props = $props();
</script>

<button class="main" class:active onclick={() => toggle(addon)}>
    <picture> </picture>

    <section class="info">
        <h2 class="title">{addon.title}</h2>
        <p class="description">{addon.description}</p>

        <section class="tags">
            <Tag>
                <CardsIcon />
                {addon.cardCount}
            </Tag>
            {#if addon.isOfficial}
                <Tag>Official</Tag>
            {/if}
        </section>
    </section>
</button>

<style lang="scss">
    @use "styles/abstracts" as *;

    .main {
        all: unset;
        border: var(--border-width) solid currentColor;
        border-radius: 96px;
        width: 100%;

        display: grid;
        grid-template-columns: min-content minmax(20px, 1fr) 30px;
        grid-template-rows: 100%;
        align-items: center;
        gap: 10px;

        transform: scale(1);
        transition: transform ease-in-out 100ms;

        &:focus-within {
            color: var(--theme-accent);
        }

        &:hover {
            color: var(--theme-accent);
        }

        @include noselect;
    }

    .active {
        transform: scale(1.03);
        color: red;
        outline: none;
    }

    picture {
        border: var(--border-width) solid currentColor;
        border-radius: 100%;
        margin-left: -var(--border-width);

        // size
        $size: clamp(64px, 10vw, 128px);
        width: $size;
        height: $size;
    }

    .info {
        display: flex;
        flex-direction: column;
        height: 100%;

        padding: 6px 0;
        font-size: clamp(1rem, 2vw, 1.7rem);

        & > .title {
            font-weight: 300;
            font-size: 1em;
            margin: 0;

            text-transform: capitalize;
            white-space: nowrap;
            text-overflow: ellipsis;
            overflow: hidden;
        }

        & > .description {
            font-weight: 400;
            font-size: 0.9rem;
            margin: 0;
        }

        & > .tags {
            margin-top: auto;
            display: flex;
            gap: 0.5em;
        }
    }
</style>
