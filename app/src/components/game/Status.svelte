<script lang="ts">
    import { WifiLogo, WifiOffLogo } from "components/icons";
    import { ConnectionStatus, type SocketState } from "lib/websocket.svelte";

    interface Props {
        socketState: SocketState;
    }

    const { socketState }: Props = $props();
</script>

{#if socketState.status != ConnectionStatus.Connected}
    <div class="connection" class:reconnecting={socketState.status == ConnectionStatus.Connecting}>
        {#if socketState.status == ConnectionStatus.Closed}
            <WifiOffLogo />
        {:else}
            <WifiLogo />
        {/if}
    </div>
{/if}

<style lang="scss">
    @keyframes reconnecting {
        from {
            opacity: 1;
        }
        to {
            opacity: 0.2;
        }
    }

    .connection {
        font-size: clamp(1rem, 2vw, 2rem);
        position: fixed;
        top: 1em;
        left: 1em;
    }

    .reconnecting {
        animation-name: reconnecting;
        animation-duration: 1s;
        animation-direction: alternate;
        animation-iteration-count: infinite;
        animation-timing-function: ease-out;
    }
</style>
