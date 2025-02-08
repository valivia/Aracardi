<script lang="ts">
    import { CardPartType, type CardController } from "routes/game/card/card.svelte";

    interface Props {
        card: CardController;
    }

    let { card }: Props = $props();
</script>

{#each card.formattedText as part}
    {#if part.type === CardPartType.text}
        {part.value}
    {:else}
        <var data-type={part.type}>
            {#if part.type === CardPartType.turns}
                {card.turns} turns
            {:else}
                {part.value}
            {/if}
        </var>
    {/if}
{/each}

<style lang="scss">
    var {
        color: var(--theme-accent);
        font-style: normal;
    }
</style>
