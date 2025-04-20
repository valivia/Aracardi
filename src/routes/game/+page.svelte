<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import { GameController, GameStage } from "lib/game.svelte";
    import AddonStage from "./stages/AddonSetup.svelte";
    import Game from "./stages/Game.svelte";
    import PlayerSetup from "./stages/PlayerSetup.svelte";
    import Settings from "./stages/Settings.svelte";
    import SettingsButton from "components/input/SettingsButton.svelte";
    import { updated } from "$app/state";
    import { onMount } from "svelte";
    import { version } from "$app/environment";

    let { data } = $props();

    let { addons } = data;

    let game: GameController = $state(new GameController(addons));

    onMount(async () => {
        const consoleStyle = "background: black;color: gold;";

        console.info("%c##########################", consoleStyle);
        console.info("%c######## Aracardi ########", consoleStyle);
        console.info("%c##### Made by Owlive #####", consoleStyle);
        console.info("%c##########################", consoleStyle);

        game.restoreSettings();
        game.restoreAddons(addons);
        await updated.check();

        console.info(`Version: ${version}`);
        console.info(`has update: ${updated.current}`);

        if (updated.current) {
            const confirmed = confirm("A new version of Aracardi is available. Do you want to update?");
            if (confirmed) {
                window.location.reload();
            }
        }
    });

    $effect(() => {
        console.log("- Settings saved");
        localStorage.setItem("settings", JSON.stringify(game.settings));
    });

    beforeNavigate(({ cancel }) => {
        if (game.currentStage !== GameStage.game) return;
        game.logGame("End");
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
