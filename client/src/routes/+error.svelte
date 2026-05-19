<script lang="ts">
    import { page } from "$app/state";
</script>

<main>
    <h1>{page.status}</h1>
    <p>{page.error?.message}</p>
    <button on:click={() => history?.back()}>Go back</button>
</main>

<style lang="scss">
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 70vh;
        padding: 1em;
    }

    h1 {
        font-size: 4em;
        margin-bottom: 0.2em;
    }

    p {
        font-size: 1.2em;
    }

    button {
        margin-top: 1em;
        background-color: transparent;
        color: currentColor;
        border: none;
        cursor: pointer;
        text-decoration: underline;
    }
</style>
