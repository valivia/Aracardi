<script>
    import Nav from "components/layout/Nav.svelte";
</script>

<div class="main">
    <slot />
</div>

<Nav />

<style lang="scss">
    @import "styles/global.scss";

    .main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
