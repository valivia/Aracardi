<script lang="ts">
    export let text: string;
</script>

<button on:click>
    {text}
</button>

<style lang="scss">
    @use "styles/abstracts" as *;

    button {
        border: var(--border-width) solid currentColor;
        border-radius: var(--border-radius);
        background-color: transparent;
        color: currentColor;
        padding: 1rem;
    }
</style>
