<svg viewBox="0 0 145.72 71">
    <!-- Right -->
    <polygon vector-effect="non-scaling-stroke" points="110.9,10.5 76.9,70.5 144.9,70.5 " />
    <!-- Middle -->
    <polygon vector-effect="non-scaling-stroke" points="72.9,60.5 106.9,0.5 38.9,0.5 " class="middle" />
    <!-- Left -->
    <polygon vector-effect="non-scaling-stroke" points="34.9,10.5 0.9,70.5 68.9,70.5 " />
</svg>

<style lang="scss">
    @use "styles/abstracts" as *;

    svg {
        fill: transparent;
        stroke: currentColor;
        stroke-width: 2px;
        cursor: pointer;
        overflow: visible;

        & > * {
            transform-origin: 50% 70%;
            transform-box: fill-box;
        }

        & .middle {
            transition: transform 150ms ease-in-out;
        }

        &:hover {
            color: var(--theme-accent);

            & .middle {
                transform: translateY(-0.5rem);
            }
        }

        &:active {
            & .middle {
                transform: translateY(0.4rem);
            }
        }
    }
</style>
