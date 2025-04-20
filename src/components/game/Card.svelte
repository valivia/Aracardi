<script lang="ts">
    import type { CardController } from "lib/card.svelte";
    import CardText from "./CardText.svelte";

    interface Props {
        card: CardController;
        loadImage: boolean;
        onclick?: () => void;
    }

    const { card, loadImage, onclick }: Props = $props();
</script>

{#key card.id}
    <button class="card" id="currentCard" {onclick}>
        {#if card.title}
            <h1 class="title underlined">{card.title}</h1>
        {/if}
        <p class="text">
            <CardText {card} />
        </p>

        {#if card.image && loadImage}
            <img
                src="/cards/{card.id}.webp"
                alt=""
                onerror={(e) => {
                    const target = e.target as HTMLImageElement;
                    if (target) target.style.display = "none";
                }}
            />
        {/if}
    </button>
{/key}

<style lang="scss">
    @use "styles/abstracts" as *;

    .card {
        isolation: isolate;

        --color: var(--theme-text);

        cursor: pointer;
        border: none;
        background: transparent;

        font-size: clamp(0.8rem, 3vw, 1.5rem);

        width: min(100%, 50ch);
        aspect-ratio: 16 / 9;
        padding: 2em;
        margin: 2em;

        border-radius: 1rem;
        overflow: hidden;
        box-shadow:
            0 0 0 2px var(--color),
            0 0 0 8px var(--theme-primary),
            0 0 0 10px var(--color),
            0 0 0 16px var(--theme-primary),
            0 0 0 18px var(--color);

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5em;

        background-position: center;
        background-size: cover;
        background-repeat: no-repeat;
        background-image: var(--background);

        animation: spin 1 200ms forwards ease-in-out;

        text-align: center;

        color: var(--theme-text);

        @include noselect;

        &:focus-visible,
        &:hover {
            outline: var(--border-width) solid var(--theme-accent);
            outline-offset: 24px;
        }

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
        font-size: 1.5em;
        font-weight: 700;
    }

    .text {
        font-size: 1em;
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
