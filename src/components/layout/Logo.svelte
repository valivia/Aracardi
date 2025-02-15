<svg viewBox="0 0 145.72 71">
    <!-- Right -->
    <polygon vector-effect="non-scaling-stroke" points="110.9,10.5 76.9,70.5 144.9,70.5 " class="left" />
    <!-- Middle -->
    <polygon vector-effect="non-scaling-stroke" points="72.9,60.5 106.9,0.5 38.9,0.5 " class="middle" />
    <!-- Left -->
    <polygon vector-effect="non-scaling-stroke" points="34.9,10.5 0.9,70.5 68.9,70.5 " class="right" />
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
            transition: transform 150ms ease-in-out;
        }

        &:hover {
            color: var(--theme-accent);

            & .right {
                transform: translate(-0.1rem, 0.1rem);
            }

            & .left {
                transform: translate(0.1rem, 0.1rem);
            }

            & .middle {
                transform: translateY(-0.5rem);
            }
        }

        &:active {
            & .right {
                transform: translate(0);
            }

            & .left {
                transform: translate(0);
            }

            & .middle {
                transform: translateY(0.4rem);
            }
        }
    }
</style>
