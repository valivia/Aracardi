<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import { GameController, GameStage } from "lib/game.svelte";
    import AddonStage from "./stages/AddonSetup.svelte";
    import Game from "./stages/Game.svelte";
    import PlayerSetup from "./stages/PlayerSetup.svelte";
    import { onDestroy, onMount } from "svelte";
    import { useSettings } from "lib/settingsContext";
    import GameSettings from "components/game/GameSettings.svelte";

    let { data } = $props();
    const { setExtras, settings } = useSettings();

    let { addons } = data;

    let game: GameController = $state(new GameController(addons, settings));

    onMount(() => {
        game.restoreAddons(addons);
    });

    beforeNavigate((navigation) => {
        if (!game.isOngoing) return;
        if (confirm("Are you sure you want to leave this page? You have unsaved changes that will be lost.")) {
            game.endGame();
        } else {
            navigation.cancel();
        }
    });

    const title = $derived.by(() => {
        switch (game.currentStage) {
            case GameStage.addonSetup:
                return " - Addons";
            case GameStage.playerSetup:
                return " - Players";
            case GameStage.game:
                return "";
            default:
                return " ??";
        }
    });

    // Settings menu
    setExtras(gameSettings);
    onDestroy(() => {
        setExtras(null);
        game.endGame();
    });
</script>

<svelte:head>
    <title>Aracardi{title}</title>
</svelte:head>

{#snippet gameSettings()}
    <GameSettings {game} />
{/snippet}

{#if game.currentStage === GameStage.addonSetup}
    <AddonStage {game} {addons} />
{:else if game.currentStage === GameStage.playerSetup}
    <PlayerSetup {game} />
{:else if game.currentStage === GameStage.game}
    <Game {game} />
{:else}
    <div>??</div>
{/if}
