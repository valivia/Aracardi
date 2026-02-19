<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import { WebsocketClient } from "lib/websocket";

    let socket = $state<WebsocketClient | null>(null);

    beforeNavigate(() => {
        socket?.close();
    });

    let current_card = $state("");
    let current_player = $state("");
    let session_id = $state("");

    async function connect() {
        socket = await WebsocketClient.connectToSession(session_id);
        socket?.onMessage((type, payload) => {
            if (type === "current_card") {
                current_card = payload;
            } else if (type === "current_player") {
                current_player = payload;
            }
        });
    }
</script>

<section>
    {#if !socket}
        <fieldset>
            <input type="text" bind:value={session_id} />
            <button onclick={connect}> Join Session </button>
        </fieldset>
    {/if}
    <fieldset>
        <legend>Current Card</legend>
        <input type="text" bind:value={current_card} />
        <button onclick={() => socket?.send("card", current_card)}>Update</button>
    </fieldset>
    <fieldset>
        <legend>Current Player</legend>
        <input type="text" bind:value={current_player} />
        <button onclick={() => socket?.send("player", current_player)}>Update</button>
    </fieldset>
</section>
