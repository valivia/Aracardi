<script lang="ts">
    import { CopyIcon } from "components/icons";

    const { data } = $props();

    function CopyToClipboard(text: string) {
        navigator.clipboard
            .writeText(text)
            .then(() => {
                console.log("Text copied to clipboard");
            })
            .catch((err) => {
                console.error("Failed to copy text: ", err);
            });
    }
</script>

<svelte:head>
    <meta name="robots" content="noindex" />
</svelte:head>

<main>
    <ul class="addons">
        {#each data.addons as addon}
            <li>
                <h2>{addon.title}</h2>
                <ul class="cards">
                    {#each addon.cards as card}
                        <li class="card">
                            <h3>{card.title}</h3>
                            <span>{card.text}</span>

                            <button onclick={() => CopyToClipboard(card.id)}>
                                <CopyIcon width="1em" height="1em" />
                            </button>

                            {#if card.image}
                                <img
                                    src="/cards/{card.id}.webp"
                                    alt=""
                                    onerror={(e) => {
                                        const target = e.target as HTMLImageElement;
                                        if (target) target.style.display = "none";
                                    }}
                                />
                            {/if}
                        </li>
                    {/each}
                </ul>
            </li>
        {/each}
    </ul>
</main>

<style lang="scss">
    main {
        padding: 1rem;
        text-align: center;
        overflow-y: auto;
    }

    h2 {
        font-size: 3rem;
        margin-bottom: 1rem;
        font-weight: 400;
    }

    .addons {
        list-style-type: none;
        display: flex;
        flex-direction: column;
        gap: 6rem;
    }

    .cards {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        justify-content: center;
        gap: 1rem;
    }

    .card {
        isolation: isolate;
        position: relative;
        overflow: hidden;

        background: transparent;
        color: currentColor;

        width: 40ch;
        height: 14rem;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        padding: 1rem;
        border-radius: 1rem;

        border: 2px solid var(--theme-text);

        & button {
            position: absolute;
            top: 0.5rem;
            right: 0.5rem;
            background: var(--theme-primary);
            padding: 0.4rem;
            border: none;
            border-radius: 0.5rem;
            color: currentColor;

            &:hover {
                color: var(--theme-accent);
            }
        }

        & h3,
        & span {
            background-color: var(--theme-primary);
            border-radius: 6px;
            padding: 0.5rem;
        }
    }

    img {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        object-fit: cover;

        z-index: -1;
    }
</style>
