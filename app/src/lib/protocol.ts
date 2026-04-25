import type { JsonPlayer } from "./player.svelte";

// Connection
export enum ConnectionClose {
    NotFound = "not_found",
    GameFull = "game_full",
    GameEnded = "game_ended",
    TimedOut = "timed_out",
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
    turns?: number;
}

export interface HostCard {
    id: string;
    players: string[];
    turns?: number;
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
