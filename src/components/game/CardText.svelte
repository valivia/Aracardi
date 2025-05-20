<script lang="ts">
    import { CardPartType, type CardController } from "lib/card.svelte";

    interface Props {
        card: CardController;
    }

    let { card }: Props = $props();
</script>

{#each card.formattedText as part}
    {#if part.type === CardPartType.text}
        {part.value}
    {:else if part.type === CardPartType.turns}
        <var data-type={part.type}>{card.turnsLeft}</var> turn{card.turnsLeft === 1 ? "" : "s"}
    {:else if part.type === CardPartType.timeLimit}
        <var data-type={part.type}>{card.timeLimit}</var> second{card.timeLimit === 1 ? "" : "s"}
    {:else}
        <var data-type={part.type}>{part.value}</var>
    {/if}
{/each}

<style lang="scss">
    var {
        color: var(--theme-accent);
        font-style: normal;
        font-weight: bold;

        &[data-type="currentPlayer"] {
            text-decoration: underline;
        }
    }
</style>
