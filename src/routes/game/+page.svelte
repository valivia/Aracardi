<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import AddonStage from "./AddonSetup.svelte";
    import Game from "./Game.svelte";
    import PlayerStage from "./PlayerSetup.svelte";
    import { GameStage, GameState } from "./state.svelte";

    // const currentPlayer = $derived.by(() => {
    //     const newPlayer = game.players[game.currentPlayerIndex];
    //     if (currentPlayer && currentPlayer.id !== newPlayer.id) {
    //         const player = document.getElementById(`player_${game.currentPlayer?.id}`);
    //         player?.scrollIntoView({ behavior: "smooth", block: "center" });
    //     }
    //     return newPlayer;
    // });

    let { data } = $props();

    let { addons } = data;

    let game = new GameState();

    beforeNavigate(({ cancel }) => {
        if (game.isClean) return;
        if (!confirm("Are you sure you want to leave this page? You have unsaved changes that will be lost.")) {
            cancel();
        }
    });

    $inspect(game.currentCard);
</script>

{#if game.currentStage === GameStage.addonSetup}
    <AddonStage {game} {addons} />
{:else if game.currentStage === GameStage.playerSetup}
    <PlayerStage {game} />
{:else if game.currentStage === GameStage.game}
    <Game {game} />
{:else}
    <div>??</div>
{/if}
