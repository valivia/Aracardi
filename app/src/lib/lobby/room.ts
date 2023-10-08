import type { Game, Session, Card } from "@prisma/client";
import { Prototype, StageState, type Stage, type SubStage } from "./stage";
import type { Player } from "./player";


/*
    NOTES
    - look into backloglimit for games without recirculation

*/

type Current = {
    player: Player;
    stage: number;
    card: Card;
};

export class GameLobby {

    // Room
    game: Game;
    session: Session;
    current: Current | null = null;

    // Player
    players: Player[] = [];

    // Card
    cards: Card[] = [];
    previousCards: Card[] = [];
    activeCards: Card[] = [];
    backlogLimit: number;


    constructor(session: Session & { game: Game }) {
        this.game = session.game;
        this.session = session;
        this.backlogLimit = Math.floor(this.cards.length * this.game.backlog_percentage);
        this.cards = [Prototype, Prototype, Prototype, Prototype, Prototype, Prototype, Prototype, Prototype, Prototype, Prototype, Prototype];
    }


    // Player
    public addPlayer(player: Player) {
        this.players.push(player);
    }

    public removePlayer(player: Player) {
        if (this.current?.player.id === player.id) throw new Error("Cannot remove current player");
        if (this.players.length < this.game.minimum_players)
            throw new Error("Not enough players to start the game");
        this.players = this.players.filter(p => p.id !== player.id);
    }


    // Processing cards

    private assignPlayersToStage(stage: Stage): Stage<StageState.inProgress> {
        if (!this.current) throw new Error("No current card");
        const availablePlayers = [...this.players];

        const subStages: SubStage<StageState.inProgress>[] = [];

        for (const subStage of stage.sub_stages) {
            const targetString = subStage.targets?.split(".");
            if (!targetString) throw new Error("No target for substage");

            const currentPlayerIndex = availablePlayers.indexOf(this.current.player);
            let targetPlayers: Player[] | undefined;

            switch (targetString[0]) {
                case "HOST":
                    targetPlayers = availablePlayers.filter(p => p.host);
                    break;
                case "CURRENT":
                    targetPlayers = availablePlayers.splice(currentPlayerIndex, 1);
                    break;
                case "NEXT": {
                    const index = (currentPlayerIndex + 1) % availablePlayers.length;
                    targetPlayers = [availablePlayers[index]];
                    break;
                }
                case "PREVIOUS": {
                    // TODO investigate if index is correct
                    const index = (currentPlayerIndex + availablePlayers.length - 1) % availablePlayers.length;
                    targetPlayers = [availablePlayers[index]];
                    break;
                }
                case "RANDOM": {
                    const randomCount = parseInt(targetString[1]);
                    if (isNaN(randomCount)) throw new Error("Invalid random count");
                    targetPlayers = [];
                    for (let i = 0; i < randomCount; i++) {
                        const randomIndex = Math.floor(Math.random() * availablePlayers.length);
                        targetPlayers.push(availablePlayers[randomIndex]);
                        availablePlayers.splice(randomIndex, 1);
                    }
                    break;
                }
                case "FILL":
                    targetPlayers = availablePlayers;
                    break;
                default:
                    break;
            }

            if (!targetPlayers) throw new Error("No target players");

            // remove target players from available players
            for (const targetPlayer of targetPlayers) {
                const index = availablePlayers.indexOf(targetPlayer);
                if (index !== -1) availablePlayers.splice(index, 1);
            }

            subStages.push({
                ...subStage,
                targets: targetPlayers
            });
        }

        return {
            ...stage,
            state: StageState.inProgress,
            sub_stages: subStages
        };
    }

    private processSubStage(stage: Stage<StageState.notStarted>) {

    }


    // Turn Logic

    private nextTurn() {
        if (!this.current) throw new Error("No current card");

        // Next Player
        const player = this.players[(this.players.indexOf(this.current.player) + 1) % this.players.length];

        // Next Card
        if (this.cards.length === 0) throw new Error("not implemented"); // TODO

        // Move current card to previous cards
        const currentCardIndex = this.cards.findIndex(c => c.id === this.current?.card.id);
        if (currentCardIndex === -1) throw new Error("Current card not found");
        this.previousCards.push(...this.cards.splice(currentCardIndex, 1));

        // Get next card
        const card = this.cards[Math.floor(Math.random() * this.cards.length)];

        // push to active cards
        // TODO

        // Recirculate cards
        if (this.previousCards.length > this.backlogLimit) {
            const card = this.previousCards.shift();
            if (!card) throw new Error("No card");
            this.cards.push(card);
        }

        this.current = {
            player,
            stage: 0,
            card,
        };
    }


    private start() {
        this.current = {
            player: this.players[0],
            stage: 0,
            card: this.cards[Math.floor(Math.random() * this.cards.length)],
        };
    }


    private nextStage() {
        if (!this.current) throw new Error("No current card");
        this.current.stage++;
    }

    public next() {
        if (!this.current) this.start();
        else if (this.current.stage < this.current.card.stages.length - 1) this.nextStage();
        else this.nextTurn();

        console.log(this.current);
    }

}



export const test = () => {
    const lobby = new GameLobby({
        join_code: "TEST",
        timer_multiplier: 1,
        turn_multiplier: 1,
        allow_nsfw: true,
        game: {
            title: "Test Game",
            description: "This is a test game",
            minimum_players: 2,
            maximum_players: 10,
            backlog_percentage: 0.8,
            allow_hotjoin: true,
        } as unknown as Game,
    } as unknown as Session & { game: Game }
    );

    lobby.addPlayer({
        id: "1",
        name: "Test",
        avatar: "",
        host: true,
    });

    lobby.addPlayer({
        id: "2",
        name: "Test",
        avatar: "",
        host: false
    });

    lobby.addPlayer({
        id: "3",
        name: "Test",
        avatar: "",
        host: false
    });

    lobby.next();
    lobby.next();
    lobby.next();
    lobby.next();
    lobby.next();
    lobby.next();
};
