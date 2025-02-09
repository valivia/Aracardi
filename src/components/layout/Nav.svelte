<script>
    import Logo from "./Logo.svelte";
</script>

<nav>
    <a href="/" class="logo">
        <Logo />
    </a>
</nav>

<style lang="scss">
    nav {
        position: absolute;
        width: 100%;
        padding: 1rem;
        bottom: 0;

        z-index: 10;

        display: flex;
        justify-content: center;
    }

    .logo {
        width: clamp(120px, 10vw, 180px);
    }
</style>
