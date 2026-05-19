<script lang="ts">
    import { CardController } from "lib/card.svelte";
    import CardText from "./CardText.svelte";
    import { DeleteIcon } from "components/icons";

    interface Props {
        card: CardController;
        onclick?: () => void;
    }

    let { card, onclick }: Props = $props();
</script>

<button {onclick} aria-label="Delete active card" disabled={onclick == undefined}>
    <span class="text">
        <CardText {card} />
    </span>
    <span class="icon">
        <DeleteIcon width="1.5em" height="1.5em" />
    </span>
</button>

<style lang="scss">
    @use "styles/abstracts" as *;
    button {
        position: relative;
        border: var(--border-width) solid currentColor;
        border-radius: var(--border-radius);
        background-color: transparent;
        color: currentColor;
        padding: 1em;
        margin: 0.5em;
        font-size: inherit;

        .icon {
            display: none;
        }

        @include noselect();

        &:hover:not(:disabled),
        &:focus-visible:not(:disabled) {
            cursor: pointer;
            outline: var(--outline-focus);
            outline-offset: var(--outline-focus-offset);

            color: var(--theme-error);
            .text {
                opacity: 0;
            }

            .icon {
                position: absolute;
                inset: 0;
                display: flex;
                align-items: center;
                justify-content: center;
            }
        }
    }
</style>
