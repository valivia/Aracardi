import type { Addon, AddonSummary } from "lib/addon";
import { CardController, CardPartType, type Card } from "./card.svelte";
import { shuffle } from "./helpers";
import { Player } from "./player.svelte";
import { nanoid } from "nanoid";
import { WebsocketClient } from "./websocket";

export enum GameStage {
    addonSetup = "addonSetup",
    playerSetup = "playerSetup",
    game = "game",
}

export interface Settings {
    allowNsfw: boolean;
    allowDuplicates: boolean;
    loadImages: boolean;
}

const defaultSettings: Settings = {
    allowNsfw: true,
    allowDuplicates: false,
    loadImages: true,
};

export class GameController {
    public readonly createdAt: Date = new Date();
    public readonly id = nanoid(32);
    public startedAt: Date | null = null;

    // Settings
    public settings: Settings = $state(defaultSettings);

    // Stage
    private stage: GameStage = $state(GameStage.playerSetup);
    public settingsOpen = $state(false);
    public get currentStage() {
        return this.stage;
    }
    public ended = false;

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
    private socket: WebsocketClient | null = $state(null);
    public readonly gameId: string | null = $derived.by(() => this.socket?.id ?? null);

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

    constructor(addons: AddonSummary[]) {
        this.selectedAddons = addons.filter((a) => a.isDefault);

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
    public deleteActiveCard = (card: CardController) => {
        this.activeCards = this.activeCards.filter((c) => c !== card);
        this.socket?.send("update", {
            activeCards: this.activeCards.map((card) => card.getHostCard()),
        });
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
        this.socket?.send("update", { players: this.players.map((player) => player.getSaveable()) });
    }

    public restorePlayers() {
        this.players = Player.loadPlayers();
    }

    public shufflePlayers = () => {
        const current = this.currentPlayer;
        this.players = shuffle(this.players);
        this.currentPlayerIndex = this.players.findIndex((p) => p === current);
    };

    // Game
    public nextTurn = () => {
        // Player
        this.setCurrentPlayer((this.currentPlayerIndex + 1) % this.players.length);

        // Active cards
        this.incrementActiveCards();

        if (this.currentCard?.turnsLeft) {
            this.activeCards.push(this.currentCard);
        }

        // Card
        this.setCurrentCard((this.currentCardIndex + 1) % this.cards.length);
        this.socket?.send("update", {
            currentPlayerId: this.currentPlayer.id,
            currentCard: this.currentCard?.getHostCard(),
            activeCards: this.activeCards.map((card) => card.getHostCard()),
        });
    };

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

    private async startGame() {
        await this.loadCards();
        if (this.cards.length < 10) {
            alert("Not enough cards");
            return;
        }

        this.startedAt = new Date();
        this.setCurrentCard(0);
        this.setCurrentPlayer(0);

        const initializeWebsocket = async () => {
            try {
                const socket = await WebsocketClient.createSession();
                this.socket = socket;
            } catch (e) {
                console.error("Failed to connect to websocket: ", e);
            }

            this.socket?.send("update", {
                players: this.players.map((player) => player.getSaveable()),
                currentPlayerId: this.currentPlayer.id,
                currentCard: { id: this.currentCard?.id, players: [] },
            });
        };

        initializeWebsocket();
    }

    public async endGame() {
        this.socket?.close();
        this.ended = true;
    }

    // Settings
    public restoreSettings() {
        const settings = localStorage.getItem("settings");
        if (settings) {
            console.info("- Settings loaded");
            this.settings = JSON.parse(settings);
        }
    }
}
