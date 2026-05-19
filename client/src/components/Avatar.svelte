<script lang="ts">
    import { avatars } from "assets/avatars/avatars.svelte";
    export let name: string;
    let avatar = avatars.find((avatar) => avatar.name === name) || avatars[0];
</script>

<div class="avatar">
    <svelte:component this={avatar.element} />
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    .avatar {
        @include avatar();
    }
</style>
