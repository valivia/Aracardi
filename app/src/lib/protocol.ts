import type { JsonPlayer } from "./player.svelte";

// Connection
export enum ConnectionClose {
    GameEnded = "GAME_ENDED",
    NotFound = "NOT_FOUND",

    GameFull = "GAME_FULL",
    TimedOut = "TIMED_OUT",

    ServerError = "SERVER_ERROR",
    ServerRestart = "SERVER_RESTART",
}

// Outgoing
export enum OutgoingMessageTopic {
    Connect = "CONNECT",
    ClientUpdate = "CLIENT_UPDATE",
    GameUpdate = "GAME_UPDATE",
}

export interface OutgoingTopicMap {
    CONNECT: string;
    CLIENT_UPDATE: ClientUpdate;
    GAME_UPDATE: HostUpdate;
}

// Incoming
export enum IncomingMessageTopic {
    GameUpdate = "GAME_UPDATE",
    ClientId = "CLIENT_ID",
}

export interface IncomingTopicMap {
    GAME_UPDATE: GameUpdate;
    CLIENT_ID: string;
}

// Data structs
export interface GameCard {
    id: string;
    title: string;
    text: string;
    image: boolean;
    players: string[];
    turnsLeft?: number;
}

export interface HostCard {
    id: string;
    instanceId: string;
    players: string[];
    turnsLeft?: number;
    turnsPassed?: number;
}

export interface ClientUpdate {
    theme?: string;
    loadImages: boolean;
    allowNsfw: boolean;
}

export interface GameUpdate {
    players?: JsonPlayer[];
    currentPlayerId?: string;

    currentCard?: GameCard;
    activeCards?: GameCard[];

    hostConnected?: boolean;
}

export interface GameInfo {
    addons: string[];
    initiatedAtMs: number;
    version: string;
}

export interface HostUpdate {
    players?: JsonPlayer[];
    currentPlayerId?: string;

    currentCard?: HostCard;
    activeCards?: HostCard[];

    info?: GameInfo;
}
