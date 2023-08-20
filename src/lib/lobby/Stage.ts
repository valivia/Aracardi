enum StageType {
    Text = "text",
    Poll = "poll",
    Input = "input"
}

interface baseStage {
    id: string;
    title?: string;
    time_limit?: number;
}

export type Stage = TextStage | PollStage | InputStage;

export interface TextStage extends baseStage {
    type: StageType.Text;
    text: string;
    has_image: boolean;
    turns: number;
}

export interface PollStage extends baseStage {
    type: StageType.Poll;
    winner_points: number;
    target: string[];
    selection_count: number;
    options: string[];
}

export interface InputStage extends baseStage {
    type: StageType.Input;
    target: string[];
}