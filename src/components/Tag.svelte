<script lang="ts">
    export let tooltip = "";
</script>

<div
    title={tooltip}
    class="main"
    class:icon={$$slots.icon}
    class:text={$$slots.default}
>
    <slot name="icon" />
    {#if $$slots.default}
        <span>
            <slot />
        </span>
    {/if}
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    $borderWidth: 1px;

    .main {
        font-size: clamp(0.75rem, 1.5vw, 1rem);
        display: flex;
        align-items: center;
    }

    .text {
        border: $borderWidth solid currentColor;
        border-radius: 100px;
    }

    span {
        padding-inline: 0.5em;
    }

    :global(.main > svg) {
        border: $borderWidth solid currentColor;
        border-radius: 100px;

        width: 2em;
        height: 2em;

        padding: 0.3em;
        margin: -$borderWidth;
    }
</style>
