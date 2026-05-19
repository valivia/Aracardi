import type { JsonPlayer } from "./player.svelte";

// Connection
export enum ConnectionClose {
    GameEnded = "GAME_ENDED",
    ServerError = "SERVER_ERROR",
    ServerRestart = "SERVER_RESTART",

    NotFound = "NOT_FOUND",
    RateLimited = "RATE_LIMITED",

    GameFull = "GAME_FULL",

    InvalidHandshake = "INVALID_HANDSHAKE",
    VersionMismatch = "VERSION_MISMATCH",

    TimedOut = "TIMED_OUT",
}

// Outgoing
export enum OutgoingMessageTopic {
    Pong = "PONG",
    Connect = "CONNECT",
    ClientUpdate = "CLIENT_UPDATE",
    GameUpdate = "GAME_UPDATE",
}

export interface OutgoingTopicMap {
    PONG: string;
    CONNECT: Connect;
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

// Message structs
export interface Connect {
    clientId?: string;
    version: string;
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

export interface HostUpdate {
    players?: JsonPlayer[];
    currentPlayerId?: string;

    currentCard?: HostCard;
    activeCards?: HostCard[];

    info?: GameInfo;
}

// Data structs
export interface GameInfo {
    addons: string[];
    initiatedAtMs: number;
    startedAtMs: number;
}

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
