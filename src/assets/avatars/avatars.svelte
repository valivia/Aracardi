<script lang="ts" context="module">
  import type { SvelteComponent } from "svelte";
  export interface Avatar {
    name: string;
    authors: string[];
    element: typeof SvelteComponent;
  }

  // Owl
  import hooty1 from "./hooty1.svelte";
  import stolas from "./stolas.svelte";
  import duo from "./duo.svelte";
  import flapjack from "./flapjack.svelte";
  import king from "./king.svelte";
  import eda from "./eda.svelte";
  import marceline from "./marceline.svelte";
  import pb from "./pb.svelte";
  import jake from "./jake.svelte";
  import finn from "./finn.svelte";
  // Czicken
  import catto from "./catto.svelte";
  import frogii from "./froggi.svelte";
  import rubberDucky from "./rubberducky.svelte";
  import duckie from "./duckie.svelte";
  import goose1 from "./goose1.svelte";
  // Pigeon
  import pidge from "./pidge.svelte";
  import goth from "./goth.svelte";
  import ghost from "./ghost.svelte";
  // Armex
  import pigeon from "./pigeon.svelte";
  import les from "./les.svelte";

  export const avatars = [
    // Owl House
    {
      name: "Hootsifer",
      authors: ["Owl"],
      element: hooty1,
    },
    {
      name: "Flapjack",
      authors: ["Owl"],
      element: flapjack,
    },
    {
      name: "King",
      authors: ["Owl"],
      element: king,
    },
    {
      name: "Eda The Owl Lady",
      authors: ["Owl"],
      element: eda,
    },

    // Adventure time
    {
      name: "Marceline",
      authors: ["Owl"],
      element: marceline,
    },
    {
      name: "Princess Bubblegum",
      authors: ["Owl"],
      element: pb,
    },
    {
      name: "Jake The Dog",
      authors: ["Owl"],
      element: jake,
    },
    {
      name: "Finn The Human",
      authors: ["Owl"],
      element: finn,
    },

    // Helluva boss
    {
      name: "Stolas",
      authors: ["Owl"],
      element: stolas,
    },

    // Animals
    {
      name: "Catto",
      authors: ["Czicken"],
      element: catto,
    },
    {
      name: "froggi",
      authors: ["Czicken"],
      element: frogii,
    },
    {
      name: "Rubber Ducky",
      authors: ["Czicken"],
      element: rubberDucky,
    },
    {
      name: "Duckie",
      authors: ["Czicken", "Lily"],
      element: duckie,
    },
    {
      name: "Goose",
      authors: ["Czicken"],
      element: goose1,
    },
    {
      name: "Pigeon",
      authors: ["Armex"],
      element: pigeon,
    },
    // Other
    {
      name: "Duo",
      authors: ["owl"],
      element: duo,
    },
    {
      name: "Pidge",
      authors: ["Pigeon"],
      element: pidge,
    },
    {
      name: "Goth",
      authors: ["Pigeon"],
      element: goth,
    },
    {
      name: "Ghost",
      authors: ["Pigeon"],
      element: ghost,
    },
    {
      name: "Les",
      authors: ["Armex"],
      element: les,
    },
  ] as Avatar[];
</script>
