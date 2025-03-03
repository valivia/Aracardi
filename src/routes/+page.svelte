<script>
    import AnchorButton from "components/input/AnchorButton.svelte";
</script>

<div class="content">
    <h1 class="title">Play now!</h1>
    <div class="divider"></div>

    <AnchorButton href="/game">Create game</AnchorButton>
</div>

<style lang="scss">
    .content {
        margin-inline: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: min(40ch, 100%);
        padding: 1rem;
    }

    .title {
        font-weight: 200;
        font-size: 2.2em;
        text-transform: capitalize;
    }

    .divider {
        width: 100%;
        height: 1px;
        margin: 1em 0;

        background-color: var(--theme-text);
    }
</style>
