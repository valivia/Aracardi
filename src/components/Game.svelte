<script lang="ts">
    import { WifiIcon, WifiOffIcon } from "svelte-feather-icons";
    import type { Game } from "@prisma/client";
    import Tag from "./Tag.svelte";
    import { slide } from "svelte/transition";
    export let game: Game;
    export const active = false;
</script>

<article class="main" tabIndex={0} class:active transition:slide|global>
    <img
        class="avatar"
        src="https://cdn.discordapp.com/attachments/808476183250993183/1135733767957401601/moon.png"
        alt=""
    />

    <section class="info">
        <h2 class="title">{game.title}</h2>
        <p class="description">{game.description}</p>

        <section class="tags">
            {#if game.is_available_online}
                <Tag><WifiIcon size="1rem" slot="icon" /></Tag>
            {/if}
            {#if game.is_available_offline}
                <Tag><WifiOffIcon size="1rem" slot="icon" /></Tag>
            {/if}
            {#if game.is_official}
                <Tag>Official</Tag>
            {/if}
        </section>
    </section>
</article>

<style lang="scss">
    @use "styles/abstracts" as *;

    $borderWidth: 1px;

    .active {
        transform: scale(1.01);
        color: $accent;
        outline: none;
    }

    .main {
        border: $borderWidth solid currentColor;
        border-radius: 96px;
        width: 100%;

        display: grid;
        grid-template-columns: min-content minmax(20px, 1fr) 30px;
        grid-template-rows: 100%;
        align-items: center;
        gap: 10px;

        transform: scale(1);
        transition: transform ease-in-out 100ms;
        cursor: pointer;

        @include noselect;

        @include pointer {
            &:hover {
                transform: scale(1.01);
                color: $accent;
            }

            &:active {
                transform: scale(1);
            }
        }
    }

    .avatar {
        border: $borderWidth solid currentColor;
        border-radius: 100%;
        margin-left: -$borderWidth;

        // size
        $size: clamp(64px, 10vw, 128px);
        width: $size;
        height: $size;
    }

    .info {
        display: flex;
        flex-direction: column;
        height: 100%;

        padding: 6px 0;
        font-size: clamp(1rem, 2vw, 1.7rem);

        & > .title {
            font-weight: 300;
            font-size: 1em;
            margin: 0;

            text-transform: capitalize;
            white-space: nowrap;
            text-overflow: ellipsis;
            overflow: hidden;
        }

        & > .description {
            font-weight: 400;
            font-size: 0.9rem;
            margin: 0;
            display: none;

            @include large {
                display: block;
            }
        }

        & > .tags {
            margin-top: auto;
            display: flex;
            gap: 0.5em;
        }
    }
</style>
