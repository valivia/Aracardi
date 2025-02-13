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
        width: 100%;
        padding: 1rem;
        margin-top: auto;

        display: flex;
        justify-content: center;
    }

    .logo {
        width: clamp(5rem, 10vw, 9rem);
    }
</style>
