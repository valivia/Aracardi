<script lang="ts">
    const { data } = $props();
</script>

<svelte:head>
    <meta name="robots" content="noindex" />
</svelte:head>

<main>
    <ul class="addons">
        {#each data.addons as addon}
            <li>
                <h2>{addon.title}</h2>
                <section class="cards">
                    {#each addon.cards as card}
                        <button onclick={() => alert(JSON.stringify(card, null, 2))}>
                            <h3>{card.title}</h3>
                            <span>{card.text}</span>

                            {#if card.image}
                                {#await import(`assets/cards/${card.id}.webp`) then { default: src }}
                                    <img {src} alt="" loading="lazy" />
                                {/await}
                            {/if}
                        </button>
                    {/each}
                </section>
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

        & button {
            isolation: isolate;
            position: relative;
            overflow: hidden;

            background: transparent;
            color: currentColor;
            cursor: pointer;

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

            & h3,
            & span {
                background-color: var(--theme-primary);
                border-radius: 6px;
                padding: 0.5rem;
            }
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
