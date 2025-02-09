<script lang="ts">
    import type { CardController } from "lib/card.svelte";
    import CardText from "./CardText.svelte";

    interface Props {
        card: CardController;
        onclick?: () => void;
    }

    const { card, onclick }: Props = $props();
</script>

{#key card.id}
    <button class="card" {onclick}>
        {#if card.title}
            <h1 class="title underlined">{card.title}</h1>
        {/if}
        <p class="text">
            <CardText {card} />
        </p>

        {#if card.hasImage}
            {#await import(`assets/cards/${card.id}.webp`) then { default: src }}
                <img {src} alt="" />
            {/await}
        {/if}
    </button>
{/key}

<style lang="scss">
    @use "styles/abstracts" as *;

    .card {
        isolation: isolate;

        cursor: pointer;
        border: none;
        background: transparent;

        width: min(100%, 50rem);
        aspect-ratio: 16 / 9;
        padding: 2rem;
        margin: 4rem;

        border-radius: 1rem;
        overflow: hidden;
        box-shadow:
            0 0 0 2px var(--theme-text),
            0 0 0 8px var(--theme-primary),
            0 0 0 10px var(--theme-text),
            0 0 0 16px var(--theme-primary),
            0 0 0 18px var(--theme-text);

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;

        background-position: center;
        background-size: cover;
        background-repeat: no-repeat;
        background-image: var(--background);

        animation: spin 200ms forwards ease-in-out;

        text-align: center;

        color: var(--theme-text);

        @include noselect;

        & img {
            position: absolute;
            width: 100%;
            height: 100%;
            object-fit: cover;

            z-index: -1;
        }
    }

    .underlined {
        text-decoration: underline;
        -webkit-text-decoration-color: var(--theme-accent);
        text-decoration-color: var(--theme-accent);
        text-decoration-thickness: 3px;
    }

    .title {
        font-size: clamp(1.5rem, 5vw, 2rem);
        font-weight: 700;
    }

    .text {
        font-size: clamp(1rem, 5vw, 1.5rem);
        font-weight: 500;
    }

    .text,
    .title {
        background-color: var(--theme-primary);
        padding: 0.2em 0.5em;
        border-radius: 0.5rem;
    }

    @keyframes spin {
        from {
            transform: scale(0.2) rotate(200deg);
        }
        to {
            transform: scale(1) rotate(0deg);
        }
    }
</style>
