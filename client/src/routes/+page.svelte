<script lang="ts">
    import { goto } from "$app/navigation";
    import { PUBLIC_SERVER_HTTP_URL } from "$env/static/public";
    import Links from "components/Links.svelte";
    import { ErrorIcon, SuccessIcon } from "components/icons";
    import AnchorButton from "components/input/AnchorButton.svelte";
    import Button from "components/input/Button.svelte";
    import { resolve } from "$app/paths";

    const statusMessages = {
        idle: "",
        checking: "Checking code…",
        valid: "Code found! Ready to join.",
        invalid: "No lobby found with that code.",
        error: "Couldn't reach the server. Try again.",
    };

    let code: string | undefined = $state(undefined);
    let status: keyof typeof statusMessages = $state("idle");
    let debounceTimer: ReturnType<typeof setTimeout> | null = null;
    let lastValidation: number = 0;
    let activeRequest: AbortController | null = null;

    async function validateCode(code: string) {
        if (code.length < 6) {
            status = "idle";
            return false;
        }

        if (activeRequest) {
            activeRequest.abort();
        }

        const controller = new AbortController();
        activeRequest = controller;

        status = "checking";
        lastValidation = Date.now();
        try {
            const res = await fetch(`${PUBLIC_SERVER_HTTP_URL}/lobby/${code}`, {
                signal: AbortSignal.any([controller.signal, AbortSignal.timeout(2000)]),
            });

            activeRequest = null;

            if (res.status == 200) {
                status = "valid";
                return true;
            } else if (res.status == 404) {
                status = "invalid";
            } else {
                status = "error";
            }
        } catch (err) {
            if (err instanceof DOMException && err.name === "AbortError") return false;

            activeRequest = null;
            status = "error";
        }
        return false;
    }

    function handleInput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        code = event.currentTarget?.value
            .toUpperCase()
            .replace(/[^A-Z0-9]/g, "")
            .slice(0, 6);

        event.currentTarget.value = code;

        if (code.length === 6) {
            if (debounceTimer) clearTimeout(debounceTimer);

            if (activeRequest) {
                activeRequest.abort();
                activeRequest = null;
            }
            status = "checking";
            if (lastValidation + 2000 < Date.now()) validateCode(code);
            else debounceTimer = setTimeout(() => validateCode(code!), 500);
        } else {
            status = "idle";
        }
    }

    async function handleJoin(e: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }) {
        if (!e.currentTarget.checkValidity()) {
            return;
        }

        e.preventDefault();
        if (status !== "valid" || (status === "valid" && Date.now() - lastValidation > 10 * 1000)) {
            if (code?.length === 6 && status !== "checking") {
                await validateCode(code!);
            }
        }
        if (status === "valid") {
            goto(resolve(`/lobby?join_code=${code}`));
        }
    }
</script>

<main>
    <section class="gameInfo">
        <p>The drinking game your friend group didn't know it needed!</p>
        <div class="divider"></div>
    </section>

    <section class="gameOptions">
        <div class="gameOption">
            <h2>Create a game</h2>
            <p>Host a game for you and your friends, in person or online!</p>
            <AnchorButton href="/game">Create Game</AnchorButton>
        </div>

        <div class="divider"></div>

        <form class="gameOption" onsubmit={handleJoin}>
            <h2>Join a game</h2>
            <div class="input">
                <label for="code"> Join code: </label>

                <span class="inputWrapper">
                    <input
                        id="code"
                        type="text"
                        name="code"
                        data-status={status}
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
                </span>

                {#if status == "error"}
                    <span>{statusMessages["error"]}</span>
                {/if}
            </div>
            <Button type="submit" disabled={status !== "valid" && code !== undefined}>Join Game</Button>
        </form>
    </section>

    <Links />
</main>

<style lang="scss">
    @use "styles/abstracts" as *;

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
        text-wrap: pretty;
    }
    .gameInfo {
        width: min(40ch, 100%);

        & p {
            text-align: center;
            font-size: 1.2em;
        }

        & .divider {
            height: 1px;
            margin-block: 1rem 3rem;
        }
    }

    .gameOptions {
        display: grid;
        grid-template-rows: auto 1px auto;
        grid-template-columns: min(40ch, 100%);
        justify-content: center;
        gap: 1.5rem;

        @include large() {
            grid-template-rows: auto;
            grid-template-columns: 30ch 1px 30ch;
            width: min-content;
        }

        .gameOption {
            display: grid;
            grid-template-rows: auto 1fr auto;
            gap: 0.5em;
            border: none;
            padding-block: 0.5em;
        }
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .spinner {
        width: 1em;
        height: 1em;

        border: 2px solid var(--theme-primary);
        border-top-color: var(--theme-text);
        border-radius: 9999px;

        animation: spin 0.6s linear infinite;
    }

    .divider {
        background-color: var(--theme-text);
        border-radius: 10px;
    }

    form {
        label {
            margin-block: 0 0.3em;
            margin-inline-start: 0.5em;
        }

        .input {
            display: grid;
        }

        .inputWrapper {
            display: flex;
            position: relative;
            isolation: isolate;

            input {
                background-color: transparent;
                border: 2px currentColor solid;
                border-radius: var(--border-radius);
                color: var(--theme-text);
                padding: 1em 1.5em;
                width: 100%;

                &:focus-visible {
                    outline: none;
                }
            }

            .input-indicator {
                position: absolute;
                display: flex;
                align-items: center;
                right: 1em;
                top: 50%;
                transform: translateY(-50%);
                pointer-events: none;
            }

            :hover,
            :focus-within {
                color: var(--theme-accent);
            }
        }
    }
</style>
