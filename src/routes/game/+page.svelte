<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import { GameController, GameStage } from "lib/game.svelte";
    import AddonStage from "./stages/AddonSetup.svelte";
    import Game from "./stages/Game.svelte";
    import PlayerSetup from "./stages/PlayerSetup.svelte";
    import Settings from "./stages/Settings.svelte";
    import SettingsButton from "components/input/SettingsButton.svelte";
    import { onMount } from "svelte";

    let { data } = $props();

    let { addons } = data;

    let game: GameController = $state(new GameController(addons));

    onMount(() => {
        game.restoreSettings();
        game.restoreAddons(addons);
    });

    $effect(() => {
        console.log("- Settings saved");
        localStorage.setItem("settings", JSON.stringify(game.settings));
    });

    beforeNavigate(({ cancel }) => {
        if (game.isClean) return;
        if (game.currentStage === GameStage.game) game.logGame("End");
        if (!confirm("Are you sure you want to leave this page? You have unsaved changes that will be lost.")) {
            cancel();
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
</script>

<svelte:head>
    <title>Aracardi{title}</title>
</svelte:head>

<SettingsButton {game} />

{#if game.settingsOpen}
    <Settings bind:game onchange={() => game.filterCards()} />
{:else if game.currentStage === GameStage.addonSetup}
    <AddonStage {game} {addons} />
{:else if game.currentStage === GameStage.playerSetup}
    <PlayerSetup {game} />
{:else if game.currentStage === GameStage.game}
    <Game {game} />
{:else}
    <div>??</div>
{/if}
