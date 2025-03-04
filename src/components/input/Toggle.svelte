<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        checked: boolean;
        children: Snippet;
    }

    let { checked = $bindable(), children }: Props = $props();
</script>

<label>
    <div class="toggle">
        <input type="checkbox" bind:checked aria-checked={checked} />
        <div class="thumb"></div>
    </div>
    <span>{@render children()}</span>
</label>

<style lang="scss">
    label {
        display: flex;
        align-items: center;
        gap: 0.5em;
    }

    .toggle {
        font-size: 2rem;
        position: relative;
        width: 2em;
        height: 1em;
        border-radius: 100vw;
        outline: 2px solid currentColor;
        transition: background-color 0.2s;
        cursor: pointer;

        &:hover {
            color: var(--theme-accent);
        }

        .thumb {
            position: absolute;
            left: 0.125em;
            top: 0.125em;
            width: 0.75em;
            height: 0.75em;
            border-radius: 100vw;
            outline: 2px solid currentColor;
            transition: transform 0.2s;
        }

        input {
            display: none;
        }

        input:checked + .thumb {
            transform: translateX(1em);
            outline: unset;
            background-color: currentColor;
        }
    }
</style>
