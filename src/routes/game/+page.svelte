<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import { GameController, GameStage } from "lib/game.svelte";
    import AddonStage from "./stages/AddonSetup.svelte";
    import Game from "./stages/Game.svelte";
    import PlayerSetup from "./stages/PlayerSetup.svelte";

    let { data } = $props();

    let { addons } = data;

    let game = new GameController();

    beforeNavigate(({ cancel }) => {
        if (game.isClean) return;
        if (!confirm("Are you sure you want to leave this page? You have unsaved changes that will be lost.")) {
            cancel();
        }
    });

</script>

{#if game.currentStage === GameStage.addonSetup}
    <AddonStage {game} {addons} />
{:else if game.currentStage === GameStage.playerSetup}
    <PlayerSetup {game} />
{:else if game.currentStage === GameStage.game}
    <Game {game} />
{:else}
    <div>??</div>
{/if}
