<script lang="ts">
    interface Props {
        name?: string;
        text: string;
        image?: string;
    }

    const { name, text, image }: Props = $props();
</script>

<div class="card" style={image ? `--background: url(${image})` : ""}>
    {#if name}
        <h1 class="title underlined">{name}</h1>
    {/if}
    <p class="text">{text}</p>
</div>

<style lang="scss">
    .card {
        width: min(100%, 50rem);
        aspect-ratio: 16 / 9;
        padding: 2rem;
        margin: 4rem;

        border-radius: 1rem;
        box-shadow:
            0 0 0 2px var(--theme-text),
            0 0 0 8px var(--theme-primary),
            0 0 0 10px var(--theme-text),
            0 0 0 16px var(--theme-primary),
            0 0 0 18px var(--theme-text);

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;

        background-position: center;
        background-size: cover;
        background-repeat: no-repeat;
        background-image: var(--background);
    }

    .underlined {
        text-decoration: underline;
        -webkit-text-decoration-color: var(--theme-accent);
        text-decoration-color: var(--theme-accent);
        text-decoration-thickness: 3px;
    }

    .title {
        font-size: clamp(1.5rem, 5vw, 2rem);
        font-weight: 700;
    }

    .text {
        font-size: clamp(1rem, 5vw, 1.5rem);
        font-weight: 500;
    }

    .text,
    .title {
        background-color: var(--theme-primary);
        padding: 0.2em 0.5em;
        border-radius: 0.5rem;
    }
</style>
