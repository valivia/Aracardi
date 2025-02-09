<script lang="ts">
    import { CardController } from "lib/card.svelte";
    import CardText from "./CardText.svelte";

    interface Props {
        card: CardController;
        onclick?: () => void;
    }

    let { card, onclick }: Props = $props();
</script>

<button {onclick}>
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
        cursor: pointer;
    }
</style>
