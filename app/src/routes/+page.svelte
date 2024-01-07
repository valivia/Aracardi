<script>
    import Button from "components/input/Button.svelte";
    import Input from "components/input/Input.svelte";
</script>

<div class="content">
    <h1 class="title">Play now!</h1>
    <div class="divider" />

    <main>
        <form>
            <Input type="text" placeholder="Join code" />
            <Button type="submit">Join Game</Button>
        </form>

        <Button>Create game</Button>
    </main>
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    .content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding-top: 20vh;
        width: min(40ch, 100%);
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

    form {
        display: flex;
    }

    main {
        display: flex;
        flex-direction: column;
        align-items: stretch;
    }
</style>
