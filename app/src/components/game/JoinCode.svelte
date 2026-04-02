<script lang="ts">
    import { page } from "$app/state";
    import { CopyIcon, InvisibleIcon, VisibleIcon } from "components/icons";

    interface Props {
        joinCode: string;
    }

    let { joinCode }: Props = $props();

    let codeVisible = $state(false);
</script>

<span class="joinCode" data-visible={codeVisible}>
    <button data-active={codeVisible} onclick={() => (codeVisible = !codeVisible)}>
        {#if codeVisible}
            <VisibleIcon />
        {:else}
            <InvisibleIcon />
        {/if}
    </button>
    <div>
        <span>Join code</span>
        <span>{codeVisible ? joinCode : "------"}</span>
    </div>
    <button onclick={async () => navigator.clipboard.writeText(`${page.url.origin}/lobby?join_code=${joinCode}`)}>
        <CopyIcon />
    </button>
</span>

<style lang="scss">
    .joinCode {
        font-size: 0.8rem;
        display: grid;
        grid-template-columns: auto auto auto;
        align-items: center;
        justify-items: center;

        border: 2px currentColor solid;
        border-radius: 100vw;

        &[data-visible="false"] > div > span:last-child {
            opacity: 0.5;
        }

        & div {
            display: grid;
            justify-items: center;
            min-width: 14ch;

            span:first-child {
                font-size: 0.8em;
            }

            & span:last-child {
                font-size: 1.1em;
                letter-spacing: 2px;
            }
        }

        & button {
            background: transparent;
            color: currentColor;

            border: 2px var(--theme-text) solid;
            border-radius: 100vw;

            padding: 0.5em;
            margin: -2px;
            font-size: 1.2em;

            display: flex;
            align-items: center;
            justify-content: center;

            &[data-active="true"] {
                background-color: var(--theme-text);
                color: var(--theme-primary);
            }

            &:hover,
            &:focus-visible {
                outline: 2px var(--theme-text) solid;
                outline-offset: 2px;
            }
        }
    }
</style>
