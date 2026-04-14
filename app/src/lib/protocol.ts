import type { JsonPlayer } from "./player.svelte";

export enum ConnectionClose {
    NotFound = "not_found",
    GameFull = "game_full",
    GameEnded = "game_ended",
    TimedOut = "timed_out",
}

export enum IncomingMessageTopic {
    Update = "update",
    ClientId = "client_id",
}

export interface IncomingTopicMap {
    update: GameUpdate;
    client_id: string;
}

export enum OutgoingMessageTopic {
    Connect = "connect",
}

export interface OutgoingTopicMap {
    update: HostUpdate;
}

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

export interface GameUpdate {
    players?: JsonPlayer[];
    currentPlayerId?: string;

    currentCard?: GameCard;
    activeCards?: GameCard[];

    hostConnected?: boolean;
}

export interface GameInfo {
    addons: string[];
    setupTimeMs: number;
    version: string;
}

export interface HostUpdate {
    players?: JsonPlayer[];
    currentPlayerId?: string;

    currentCard?: HostCard;
    activeCards?: HostCard[];

    info?: GameInfo;
}
