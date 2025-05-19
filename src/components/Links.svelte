<script>
    import { PUBLIC_DISCORD_URL, PUBLIC_DONATION_URL } from "$env/static/public";
    import { CoffeeIcon } from "components/icons";
    import DiscordLogo from "./icons/DiscordLogo.svelte";
</script>

<aside>
    <a href={PUBLIC_DONATION_URL} target="_blank" rel="noopener">
        <CoffeeIcon width="1.4em" height="1.4em" /><span>Buy me a cup of tea!</span>
    </a>

    <a href={PUBLIC_DISCORD_URL} target="_blank" rel="noopener">
        <DiscordLogo /><span>Join the Discord</span>
    </a>
</aside>

<style lang="scss">
    aside {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        margin: 1em 0;


    }

    a {
        display: inline-flex;
        align-items: center;
        margin-block: 0.4em;
        text-decoration: underline;

        span {
            margin-inline: 0.5em;
        }

        &:hover {
            color: var(--theme-accent);
        }

        :global(svg) {
            width: 1.4em;
            height: 1.4em;
        }
    }
</style>
