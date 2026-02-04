<script lang="ts">
    import { beforeNavigate } from "$app/navigation";

    const socket = $state(new WebSocket("ws://localhost:3000/ws/1234"));
    socket.addEventListener("open", function (event) {
        console.log("socket open");
        const player_id = localStorage.getItem("player_id");
        const connectMessage = `connect:${player_id ?? ""}`;
        console.log("Sending message: ", connectMessage);
        socket.send(connectMessage);
    });

    // Listen for messages
    socket.addEventListener("message", function (event) {
        const data: string = event.data;
        console.log("Message from server ", data);

        if (data.startsWith("player_id:")) {
            const playerId = data.split(":")[1];
            console.log("Received player ID: ", playerId);
            localStorage.setItem("player_id", playerId);
        }
    });

    beforeNavigate(() => {
        console.log("closing socket");
        socket.close();
    });
</script>

<section>
    <button onclick={() => socket.send("a")}>Send Message</button>
</section>
