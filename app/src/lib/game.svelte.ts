import type { Addon, AddonSummary } from "lib/addon";
import { CardController, type Card } from "./card.svelte";
import { shuffle } from "./helpers";
import { Player } from "./player.svelte";
import { nanoid } from "nanoid";
import { WebsocketClient } from "./websocket.svelte";
import { version } from "$app/environment";
import { defaultSettings, type Settings } from "./settingsContext";
import type { Unsubscriber, Writable } from "svelte/store";
import { IncomingMessageTopic, OutgoingMessageTopic } from "./protocol";
import { HAS_TRIED_THEMES_KEY } from "components/theme";

export enum GameStage {
    addonSetup = "addonSetup",
    playerSetup = "playerSetup",
    game = "game",
}

const FAKE_SESSION_CREATION_TIME_MS = 3000;

export class GameController {
    public readonly createdAt: Date = new Date();
    public readonly id = nanoid(32);
    public startedAt: Date | null = null;

    // Stage
    private stage: GameStage = $state(GameStage.playerSetup);
    public settingsOpen = $state(false);
    public get currentStage() {
        return this.stage;
    }
    public ended = false;

    public settings: Settings = $state(defaultSettings);
    private unsubscribeSettings: Unsubscriber;

    // Content
    public cards: Card[] = $state([]);
    public disabledCards: Card[] = $state([]);
    public players: Player[] = $state([]);

    // Current player
    public currentPlayerIndex: number = $state(0);
    public get currentPlayer(): Player {
        return this.players[this.currentPlayerIndex];
    }

    // Current card
    private currentCardIndex: number = $state(0);
    public currentCard: CardController | null = $state(null);

    // Active cards
    public activeCards: CardController[] = $state([]);

    // Websocket
    public socket: WebsocketClient | null = $state(null);
    public readonly joinCode: string | null = $derived.by(() => this.socket?.session_id ?? null);
    public connectingTimeout?: ReturnType<typeof setTimeout>;
    public isConnecting = $state(false);
    public isDismissed = $state(false);
    private settingsStore: Writable<Settings>;
    private unsubscribeSocketSettings: Unsubscriber | undefined;

    // Setup
    public selectedAddons: AddonSummary[] = $state([]);

    public hasPreviousPlayers = false;

    // Helpers
    public projectedCardCount = $derived.by(() => {
        return this.selectedAddons.reduce((acc, addon) => {
            return acc + addon.cardCount - (this.settings.allowNsfw ? 0 : addon.nsfwCardCount);
        }, 0);
    });

    public isOngoing = $derived.by(() => {
        return this.currentCard !== null;
    });

    constructor(addons: AddonSummary[], settingsStore: Writable<Settings>) {
        this.selectedAddons = addons.filter((a) => a.isDefault);
        this.settingsStore = settingsStore;

        this.unsubscribeSettings = settingsStore.subscribe((updated) => {
            const prev = this.settings;

            const nsfwChanged = prev.allowNsfw !== updated.allowNsfw;
            const dupChanged = prev.allowDuplicates !== updated.allowDuplicates;

            this.settings = { ...updated };

            if (nsfwChanged || dupChanged) {
                this.filterCards();
            }
        });

        // Check if client side
        if (typeof window !== "undefined") this.hasPreviousPlayers = localStorage.getItem("players") !== null;
    }

    // Addons
    public toggleAddon = (addon: AddonSummary) => {
        const index = this.selectedAddons.findIndex((a) => a.id === addon.id);
        if (index === -1) {
            this.selectedAddons.push(addon);
        } else {
            this.selectedAddons.splice(index, 1);
        }

        this.saveAddons();
    };

    public hasAddon = (addon: AddonSummary) => {
        return this.selectedAddons.some((a) => a.id === addon.id);
    };

    private saveAddons() {
        const savedAddons = this.selectedAddons.map((a) => a.id);
        localStorage.setItem("addons", JSON.stringify(savedAddons));
    }

    public restoreAddons(availableAddons: AddonSummary[]) {
        const json = localStorage.getItem("addons");
        if (json) {
            const savedAddons = JSON.parse(json) as string[];
            console.info(`- Addons loaded (${savedAddons.length})`);
            const selectedAddons = availableAddons.filter((addon) => savedAddons.some((id) => id === addon.id));
            this.selectedAddons = selectedAddons;
        }
    }

    // Cards
    public loadCards = async () => {
        const addons = await Promise.all(
            this.selectedAddons.map(async (addonInfo) => {
                const json = await import(`assets/addons/${addonInfo.fileName}.json`);
                const addon = json.default as Addon;
                return addon;
            }),
        );

        this.cards = shuffle(addons.flatMap((addon) => addon.cards));
        this.filterCards();
    };

    public filterCards = () => {
        // TODO: filter out current card
        const allCards = [...this.cards, ...this.disabledCards];

        type SeparatedCards = {
            validCards: Card[];
            invalidCards: Card[];
        };

        const overrideIds: Set<string> = new Set(allCards.flatMap((card) => card.overrides || []));

        // Filter min and max players
        const { validCards, invalidCards } = allCards.reduce<SeparatedCards>(
            (acc, card) => {
                let isInvalid = false;

                // Game settings
                isInvalid =
                    isInvalid ||
                    (card.isNsfw === true && !this.settings.allowNsfw) ||
                    (!this.settings.allowDuplicates && overrideIds.has(card.id));

                // Players
                isInvalid =
                    isInvalid ||
                    (card.minPlayers !== undefined && this.players.length < card.minPlayers) ||
                    (card.maxPlayers !== undefined && this.players.length > card.maxPlayers);

                // Push to correct array
                if (isInvalid) {
                    acc.invalidCards.push(card);
                } else {
                    acc.validCards.push(card);
                }

                return acc;
            },
            { validCards: [], invalidCards: [] },
        );

        console.log(`- Filtered cards: ${validCards.length} valid, ${invalidCards.length} invalid`);

        this.cards = validCards;
        this.disabledCards = invalidCards;
    };

    // Active cards
    public deleteActiveCard = (card: CardController, notifyServer = false) => {
        this.activeCards = this.activeCards.filter((c) => c !== card);
        if (notifyServer) {
            this.socket?.send(IncomingMessageTopic.GameUpdate, {
                activeCards: this.activeCards.map((card) => card.getHostCard()),
            });
        }
    };

    private incrementActiveCards() {
        for (const card of this.activeCards) {
            card.nextTurn();

            if (card.turnsLeft === 0) {
                this.deleteActiveCard(card);
            }
        }
    }

    // Players
    public getPlayer(id: string) {
        return this.players.find((p) => p.id === id);
    }

    public upsertPlayer = (player: Player) => {
        const index = this.players.findIndex((p) => p.id === player.id);

        if (index === -1) {
            this.players.push(player);
            this.filterCards();
        } else {
            this.players[index] = player;
        }

        this.savePlayers();
    };

    public removePlayer = (player: Player) => {
        const newPlayers = this.players.filter((p) => p !== player);
        if (newPlayers.length === this.players.length) return;

        this.players = newPlayers;

        this.savePlayers();
        this.filterCards();

        this.activeCards = this.activeCards.filter((card) => !card.players.has(player));
    };

    private savePlayers() {
        Player.savePlayers(this.players);
        this.hasPreviousPlayers = this.players.length > 0;

        // Log if in game
        this.socket?.send(IncomingMessageTopic.GameUpdate, {
            players: this.players.map((player) => player.getSaveable()),
        });
    }

    public restorePlayers() {
        this.players = Player.loadPlayers();
    }

    public shufflePlayers() {
        const current = this.currentPlayer;
        this.players = shuffle(this.players);
        this.currentPlayerIndex = this.players.findIndex((p) => p === current);
    }

    // Game
    public nextTurn() {
        // Player
        this.setCurrentPlayer((this.currentPlayerIndex + 1) % this.players.length);

        // Active cards
        this.incrementActiveCards();

        if (this.currentCard?.turnsLeft) {
            this.activeCards.push(this.currentCard);
        }

        // Card
        this.setCurrentCard((this.currentCardIndex + 1) % this.cards.length);
        this.socket?.send(IncomingMessageTopic.GameUpdate, {
            currentPlayerId: this.currentPlayer.id,
            currentCard: this.currentCard?.getHostCard(),
            activeCards: this.activeCards.map((card) => card.getHostCard()),
        });
    }

    private setCurrentPlayer(index: number) {
        this.currentPlayerIndex = index;
    }

    private setCurrentCard(index: number) {
        this.currentCardIndex = index;
        this.currentCard = CardController.fromHostCard(
            this.cards[this.currentCardIndex],
            [...this.players],
            this.currentPlayerIndex,
        );
    }

    public async setStage(state: GameStage) {
        if (state === GameStage.game && !this.currentCard) {
            this.startGame();
        }

        this.stage = state;
    }

    public async initializeWebsocket() {
        let socket;

        this.isConnecting = true;
        this.connectingTimeout = undefined;
        try {
            socket = await WebsocketClient.createSession();
        } catch (e) {
            console.error("Failed to initialize websocket: ", e);
            this.connectingTimeout = setTimeout(() => {
                this.isConnecting = false;
                console.log("bbbb");
            }, FAKE_SESSION_CREATION_TIME_MS);
            return;
        }

        this.isConnecting = false;
        this.isDismissed = false;

        this.socket = socket;

        this.socket?.send(IncomingMessageTopic.GameUpdate, {
            players: this.players.map((player) => player.getSaveable()),
            currentPlayerId: this.currentPlayer.id,
            currentCard: this.currentCard?.getHostCard(),
            activeCards: this.activeCards.map((card) => card.getHostCard()),
            info: {
                addons: this.selectedAddons.map((addon) => addon.title),
                initiatedAtMs: Number(this.createdAt),
                startedAtMs: Number(this.startedAt),
                version: version,
            },
        });

        this.unsubscribeSocketSettings?.();
        this.unsubscribeSocketSettings = this.settingsStore.subscribe((value) => {
            this.socket?.send(OutgoingMessageTopic.ClientUpdate, {
                theme: localStorage.getItem(HAS_TRIED_THEMES_KEY) === "true" ? value.theme : undefined,
                loadImages: value.loadImages,
                allowNsfw: value.allowNsfw,
            });
        });
    }

    private async startGame() {
        await this.loadCards();
        if (this.cards.length < 10) {
            alert("Not enough cards");
            return;
        }

        this.startedAt = new Date();
        this.setCurrentCard(0);
        this.setCurrentPlayer(0);

        this.initializeWebsocket();
    }

    public async endGame() {
        if (this.ended) return;

        this.ended = true;
        this.unsubscribeSettings();
        this.unsubscribeSocketSettings?.();
        this.socket?.close();

        console.info("game ended");
    }
}
