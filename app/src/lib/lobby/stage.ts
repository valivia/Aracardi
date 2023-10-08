import type { Card as PrismaCard } from "@prisma/client";
import type { Player } from "./player";


// Stage

export enum StageState {
    notStarted = "NOT_STARTED",
    inProgress = "IN_PROGRESS",
    completed = "COMPLETED",
}

export type Stage<State extends StageState = StageState.notStarted> = {
    state: State;
    time_limit?: number;
    sub_stages: SubStage<State>[];
}

// SubStage
export enum SubStageType {
    Text = "TEXT",
    Poll = "POLL",
    Input = "INPUT",
    Empty = "EMPTY"
}

type baseSubStage<State extends StageState = StageState.notStarted> = {
    id: string;
    title?: string;
    targets: State extends StageState.notStarted ? PlayerSelector : Player[]
}

export interface EmptySubStage<T extends StageState> extends baseSubStage<T> {
    type: SubStageType.Empty;
}

export interface TextSubStage<T extends StageState> extends baseSubStage<T> {
    type: SubStageType.Text;
    text: string;
    has_image: boolean;
}

export interface InputSubStage<T extends StageState> extends baseSubStage<T> {
    type: SubStageType.Input;
    placeholder: string;
    answers?: T extends StageState.completed ? string[] : null;
}

export interface PollSubStage<T extends StageState> extends baseSubStage<T> {
    type: SubStageType.Poll;
    selection_count?: number;
    options: PollOption[];
    winner?: T extends StageState.completed ? string[] : null;
}

export type PollOption = {
    id: string;
    text: string; // Can be a template string, if value is an array it will create multiple options
    points: number;
    correct: boolean;
};

export type SubStage<T extends StageState = StageState.notStarted> =
    TextSubStage<T> |
    PollSubStage<T> |
    InputSubStage<T> |
    EmptySubStage<T>;


type Card = PrismaCard & {
    stages: Stage[];
}
export const Prototype: Card = {
    id: "clljm9y0o001pvdbpx8456emi",
    addon_id: "clljm9y0p001rvdbpafjexu3a",
    created_at: new Date(),
    updated_at: new Date(),

    minimum_players: 4,
    maximum_players: 8,

    is_available_offline: false,
    is_available_online: true,
    is_nsfw: false,

    stages: [
        // Stage 1
        {
            state: StageState.notStarted,
            time_limit: 60,
            sub_stages: [
                {
                    type: SubStageType.Text,
                    id: "clljm9y0o001dvdbp61euwpb1",
                    text: "This is a text stage",
                    title: "Text Stage",
                    has_image: false,
                    targets: "CURRENT",

                },
                {
                    type: SubStageType.Input,
                    id: "clljm9y0o001fvdbppf5pv952",
                    title: "Enter your favourite type of bird",
                    placeholder: "e.g. Robin",
                    targets: "FILL.MIN.3",
                }
            ]
        },
        // Stage 2
        {
            state: StageState.notStarted,
            sub_stages: [
                {
                    type: SubStageType.Poll,
                    id: "clljm9y0o001hvdbpicwuog2c",
                    title: "vote on the best bird",
                    targets: "FILL",
                    options: [
                        {
                            id: "clljm9y0o001jvdbpp1c7h5eq",
                            text: "{stage(1.2).output}", // This will create atleast 3 options with the user input from stage 1.2
                            points: 0,
                            correct: false,
                        },
                        {
                            id: "clljm9y0o001nvdbplxbkq9ct",
                            text: "{stage.1.1.targets}",
                            points: 100,
                            correct: true,
                        },
                    ]
                }
            ],
        }
    ]
};


/* Text templating
    - {host} - Host name
    - {current} - Current player name
    - {previous} - Previous player name
    - {next} - Next player name
    - {stage(1.1)...}
    - {stage(2.3)....}  

    stage specific fields
    base:
        - {.targets} - string or array of player names
    poll:
        - {.options} - string or array of Poll options
        - {.winner} - Winner(s) of poll
        - {.correct} - Correct answer(s)
    input:
        - {.output} - Data entered by player
*/

export type PlayerSelector = "HOST"
    | "CURRENT"
    | "PREVIOUS"
    | "NEXT"
    | "FILL"
    | `FILL.MIN.${string}`
    | `FILL.MAX.${string}`
    | `RANDOM.${string}`;

// what if missing value from previous stage?
// Data integrity with stored minimium and maximum players
// multi turn effects??
// Solution to mixing HOST with other selectors
