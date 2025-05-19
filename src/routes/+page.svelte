<script>
    import Links from "components/Links.svelte";
    import AnchorButton from "components/input/AnchorButton.svelte";
</script>

<main>
    <p>Have an unforgettable drinking night with your friends on Aracardi!</p>
    <div class="divider"></div>
    <AnchorButton href="/game">Play</AnchorButton>
    <Links />
</main>

<style lang="scss">
    main {
        position: relative;
        margin-inline: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: min(40ch, 100%);
        padding: 1rem;

        & > :global(:last-child) {
            position: absolute;
            bottom: 1em;
        }
    }

    p {
        text-align: center;
        font-size: 1.2em;
    }

    .divider {
        width: 100%;
        height: 1px;
        margin: 1em 0;

        background-color: var(--theme-text);
    }
</style>
