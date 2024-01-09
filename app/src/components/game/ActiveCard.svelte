<script lang="ts">
    export let text: string;
</script>

<button on:click>
    {text}
</button>

<style lang="scss">
    @use "styles/abstracts" as *;

    button {
        border: $border;
        border-radius: $borderRadius;
        background-color: transparent;
        color: currentColor;
        padding: 1rem;
    }
</style>
