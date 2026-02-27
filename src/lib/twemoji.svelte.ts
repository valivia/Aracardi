import api, { type TwemojiOptions } from "@twemoji/api";

const twemojiOptions: TwemojiOptions = {
    className: "twemoji",
};

export function twemoji(node: HTMLElement) {
    api.parse(node, twemojiOptions);
    $effect(() => {
        api.parse(node, twemojiOptions);
    })
}
