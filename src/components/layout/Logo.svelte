<svg viewBox="0 0 145.72 71">
    <polygon points="110.86 10.5 76.86 70.5 144.86 70.5 110.86 10.5" />
    <polygon
        class="inner"
        points="110.86 20.1 85.46 64.9 136.26 64.9 110.86 20.1"
    />
    <polygon points="72.86 60.5 106.86 .5 38.86 .5 72.86 60.5" />
    <polygon points="34.86 10.5 .86 70.5 68.86 70.5 34.86 10.5" />
</svg>

<style lang="scss">
    @use "styles/abstracts" as *;

    svg {
        fill: transparent;
        stroke: currentColor;
        stroke-width: 1px;
        cursor: pointer;

        & > * {
            transform-origin: center;
            transform-box: fill-box;
        }

        transition: transform 0.2s ease-in-out;
        transform: scale(1);

        &:hover {
            color: $accent;
            transform: scale(1.1);
        }
    }
</style>
