<script>
    import Logo from "./Logo.svelte";
</script>

<nav>
    <div class="logo">
        <Logo />
    </div>
</nav>

<style lang="scss">
    nav {
        position: absolute;
        width: 100%;
        padding: 1rem;
        bottom: 0;

        display: flex;
        justify-content: center;
    }

    .logo {
        width: min(100%, 200px);
    }
</style>
