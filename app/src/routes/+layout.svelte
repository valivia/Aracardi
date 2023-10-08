<script>
    import Nav from "components/layout/Nav.svelte";
</script>

<div class="wrapper">
    <h1>Aracardi</h1>
    <slot />
</div>
<Nav />

<style lang="scss">
    @import "styles/global.scss";

    h1 {
        padding: 0.2em;
        text-align: center;
    }

    .wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 1em;
    }
</style>
