<script>
    import Logo from "./Logo.svelte";
</script>

<nav>
    <a href="/" class="logo" aria-label="Go to home page" id="homeButton">
        <Logo />
    </a>
</nav>

<style lang="scss">
    @use "styles/abstracts" as *;

    nav {
        width: 100%;
        padding: 1rem;
        margin-top: auto;

        display: none;
        justify-content: center;

        @include large() {
            display: flex;
        }
    }

    .logo {
        width: clamp(5rem, 10vw, 9rem);

        &:focus-visible {
            outline: none;

            border-bottom: 2px solid var(--theme-accent);
            margin-bottom: -2px;

            :global(svg) {
                color: var(--theme-accent);
            }
        }
    }
</style>
