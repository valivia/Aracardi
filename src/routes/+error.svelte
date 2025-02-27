<script lang="ts">
    import { page } from "$app/state";
</script>

<main>
    <h1>{page.status}</h1>
    <p>{page.error?.message}</p>
    <button on:click={() => history?.back()}>Go back</button>
</main>

<style lang="scss">
    @use "./404/404.scss" as *;
</style>
