<script lang="ts">
    import { page } from "$app/state";
    import { CopyIcon, InvisibleIcon, VisibleIcon } from "components/icons";

    interface Props {
        joinCode: string;
        disabled: boolean;
    }

    let { joinCode, disabled = false }: Props = $props();

    let codeVisible = $state(false);

    $effect(() => {
        if (disabled == true) codeVisible = false;
    });
</script>

<span class="joinCode" data-visible={codeVisible} data-disabled={disabled}>
    <button data-active={codeVisible} onclick={() => (codeVisible = !codeVisible)} {disabled}>
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
    <button
        onclick={async () => await navigator.clipboard.writeText(`${page.url.origin}/lobby?join_code=${joinCode}`)}
        {disabled}
    >
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

        &[data-disabled="true"] {
            opacity: 0.5;
        }

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

            &:hover:not(:disabled),
            &:focus-visible:not(:disabled) {
                outline: 2px var(--theme-text) solid;
                outline-offset: 2px;
            }
        }
    }
</style>
