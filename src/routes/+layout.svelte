<script>
    import Nav from "components/layout/Nav.svelte";
</script>

<h1>Aracardi</h1>
<div class="main">
    <slot />
</div>

<Nav />

<style lang="scss">
    @import "styles/global.scss";

    h1 {
        padding: 0.2em;
        text-align: center;
    }

    .main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
