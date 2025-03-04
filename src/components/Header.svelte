<script lang="ts">
    interface Props {
        title: string;
        subtitle?: string;
    }

    let { title, subtitle }: Props = $props();
</script>

<header>
    <h2>{title}</h2>
    {#if subtitle}
        <p>{subtitle}</p>
    {/if}
</header>

<style lang="scss">
    header {
        display: flex;
        flex-direction: column;
        gap: 1rem;

        margin-inline: auto;
        text-align: center;
        max-width: min(100%, 40ch);

        font-size: clamp(0.8rem, 2vw, 1rem);

        h2 {
            font-size: 1.5em;
            font-weight: 600;
        }

        p {
            font-size: 1em;
        }
    }
</style>
