<script>
    import { PUBLIC_DONATION_URL } from "$env/static/public";
    import { CoffeeIcon } from "lib/icons";
</script>

<a href={PUBLIC_DONATION_URL} target="_blank" rel="noopener">
    <CoffeeIcon width="1.4em" height="1.4em" /><span>Buy me a cup of tea!</span>
</a>

<style lang="scss">
    a {
        margin: 1em 0;
        margin-inline: auto;

        display: inline-flex;
        align-items: center;

        span {
            margin-inline: 0.5em;
        }

        &:hover {
            text-decoration: underline;
            color: var(--theme-accent);
        }
    }
</style>
