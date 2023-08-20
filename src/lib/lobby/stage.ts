export enum StageType {
    Text = "text",
    Poll = "poll",
    Input = "input"
}

type baseStage = {
    id: string;
    title?: string;
    time_limit?: number;
}

export type Stage = TextStage | PollStage | InputStage;

export type TextStage = baseStage & {
    type: StageType.Text;
    text: string;
    has_image: boolean;
    turns: number;
}

export type PollStage = baseStage & {
    type: StageType.Poll;
    winner_points: number;
    target: string[];
    selection_count: number;
    options: string[];
}

export type InputStage = baseStage & {
    type: StageType.Input;
    target: string[];
}