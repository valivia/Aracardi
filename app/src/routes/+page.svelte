<script lang="ts">
    import { PUBLIC_SERVER_HTTP_URL } from "$env/static/public";
    import Links from "components/Links.svelte";
    import { ErrorIcon, SuccessIcon } from "components/icons";
    import AnchorButton from "components/input/AnchorButton.svelte";
    import Button from "components/input/Button.svelte";

    const statusMessages = {
        idle: "",
        checking: "Checking code…",
        valid: "Code found! Ready to join.",
        invalid: "No lobby found with that code.",
        error: "Couldn't reach the server. Try again.",
    };

    let code = $state("");
    let status: keyof typeof statusMessages = $state("idle");

    async function validateCode(code: string) {
        if (code.length < 6) {
            status = "idle";
            return false;
        }

        status = "checking";
        try {
            const res = await fetch(`${PUBLIC_SERVER_HTTP_URL}/lobby/${code}`);
            if (res.status == 200) {
                status = "valid";
                return true;
            } else if (res.status == 404) {
                status = "invalid";
            } else {
                status = "error";
            }
        } catch {
            status = "error";
        }
        return false;
    }

    function handleInput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        code = event.currentTarget?.value
            .toUpperCase()
            .replace(/[^A-Z0-9]/g, "")
            .slice(0, 6);

        if (code.length === 6) {
            validateCode(code);
        } else {
            status = "idle";
        }
    }

    async function handleJoin(e: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }) {
        e.preventDefault();
        if (status !== "valid") {
            if (code.length === 6 && status !== "checking") {
                await validateCode(code);
            }
        }
        if (status === "valid") {
            window.location.href = `/lobby?join_code=${code}`;
        }
    }
</script>

<main>
    <section class="gameInfo">
        <p>Have an unforgettable drinking night with your friends on Aracardi!</p>
        <div class="divider"></div>
    </section>

    <section class="gameOptions">
        <fieldset>
            <h2>Create a game</h2>
            <AnchorButton href="/game">Create Game</AnchorButton>
        </fieldset>

        <form onsubmit={handleJoin}>
            <h2>Join a game</h2>
            <div class="input">
                <label for="code"> Join code: </label>

                <input
                    id="code"
                    type="text"
                    name="code"
                    bind:value={code}
                    oninput={handleInput}
                    placeholder="ABC123"
                    autocomplete="off"
                    spellcheck="false"
                    minlength="6"
                    maxlength="6"
                    required
                />

                <div class="input-indicator" aria-hidden="true">
                    {#if status === "checking"}
                        <span class="spinner"></span>
                    {:else if status === "valid"}
                        <SuccessIcon width="1em" />
                    {:else if status === "invalid" || status === "error"}
                        <ErrorIcon width="1em" />
                    {/if}
                </div>
                {#if statusMessages[status]}
                    <p class="status-msg" data-status={status} role="status">
                        {statusMessages[status]}
                    </p>
                {/if}
            </div>
            <Button type="submit">Join Game</Button>
        </form>
    </section>

    <Links />
</main>

<style lang="scss">
    .gameInfo {
        width: min(40ch, 100%);
    }

    .gameOptions {
        display: flex;
        flex-wrap: wrap;
        gap: 2rem;
    }

    .input {
        display: grid;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .spinner {
        width: 1em;
        height: 1em;
        border: 2px solid var(--border);
        border-top-color: var(--accent);
        border-radius: 50%;
        animation: spin 0.6s linear infinite;
    }

    fieldset,
    form {
        display: flex;
        flex-direction: column;
        min-width: 40ch;
        gap: 0.5em;
        border: none;
        padding-block: 0.5em;
        margin-inline: auto;
    }

    label {
        display: grid;
    }

    input {
        background-color: transparent;
        border: 2px currentColor solid;
        border-radius: var(--border-radius);
        color: var(--theme-text);
        padding: 1em 1.5em;

        &:focus-visible {
            color: var(--theme-accent);
            outline: none;
        }
    }

    main {
        position: relative;
        margin-inline: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 1rem;

        & > :global(:last-child) {
            position: absolute;
            bottom: 1em;
        }
    }

    p {
        text-align: center;
        font-size: 1.2em;
    }

    .divider {
        width: 100%;
        height: 1px;
        margin: 1em 0;

        background-color: var(--theme-text);
    }
</style>
