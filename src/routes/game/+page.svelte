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
        let settings = localStorage.getItem("settings");
        if (settings) {
            console.log("- Settings loaded");
            game.settings = JSON.parse(settings);
        }
    });

    $effect(() => {
        console.log("- Settings saved");
        localStorage.setItem("settings", JSON.stringify(game.settings));
    });

    beforeNavigate(({ cancel }) => {
        if (game.isClean) return;
        if (!confirm("Are you sure you want to leave this page? You have unsaved changes that will be lost.")) {
            cancel();
        }
    });
</script>

{#if game.settingsOpen}
    <Settings bind:game />
{:else if game.currentStage === GameStage.addonSetup}
    <AddonStage {game} {addons} />
{:else if game.currentStage === GameStage.playerSetup}
    <PlayerSetup {game} />
{:else if game.currentStage === GameStage.game}
    <Game {game} />
{:else}
    <div>??</div>
{/if}

{#if game.currentStage !== GameStage.game}
    <SettingsButton {game} />
{/if}
