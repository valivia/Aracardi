<script>
    import Logo from "./Logo.svelte";
</script>

<nav>
    <Logo />
</nav>

<style lang="scss">
    @use "styles/abstracts" as *;

    nav {
        width: clamp(5rem, 10vw, 9rem);
        padding: 1rem;
        margin-top: auto;
        margin-inline: auto;

        display: none;

        @include large() {
            display: flex;
        }

        :global(svg) {
            width: 100%;
            height: 100%;
        }
    }
</style>
