<script lang="ts">
    import type { CardController } from "routes/game/card/card.svelte";
    import CardText from "./CardText.svelte";

    interface Props {
        card: CardController;
    }

    let { card }: Props = $props();
</script>

<button>
    <CardText {card} />
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
