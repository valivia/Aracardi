import { Addon, Game, PrismaClient, Role } from "@prisma/client";
import { faker } from "@faker-js/faker";
import { PollStage, TextStage, StageType } from "../src/lib/lobby/Stage";

const prisma = new PrismaClient();
const createArray = (min: number, max: number) => Array(faker.datatype.number({ min, max })).fill(2);

function createTextStage(): TextStage {
  const a = faker.lorem
    .sentence()
    .split(" ")
    .map((x) => (Math.random() < 0.1 ? faker.helpers.arrayElement(["%PLAYER1%", "%SELF%", "%TURNS%"]) : x));

  const hasTurns = a.indexOf("%TURNS%") !== -1;
  const text = a.join(" ");

  return {
    type: StageType.Text,
    title: faker.random.words(5),
    text,
    turns: hasTurns ? faker.datatype.number({ min: 1, max: 12 }) : null,
  } as TextStage;
}

function createPollStage(): PollStage {
  return {
    type: StageType.Poll,
    title: faker.random.words(5),
    winner_points: faker.datatype.boolean() ? faker.datatype.number({ min: 100, max: 1000, precision: 100 }) : null,
    selection_count: 1,
  } as PollStage;
}

function createCard(addon: Addon) {
  const is_available_online = faker.datatype.boolean();
  const stages = createArray(1, 5).map(() => (faker.datatype.boolean() ? createTextStage() : createPollStage()));
  const query = prisma.card.create({
    data: {
      addon_id: addon.id,
      is_nsfw: faker.datatype.boolean(),
      is_available_online,
      stages,
      is_available_offline: is_available_online ? faker.datatype.boolean() : true,
    },
  });

  return query;
}

function createAddon(game: Game): Omit<Addon, "id" | "author_ids"> {
  return {
    created_at: faker.date.past(2),
    updated_at: faker.date.past(),

    title: faker.random.words(2),
    description: faker.lorem.sentence(),

    has_image: faker.datatype.boolean(),
    is_official: Math.random() < 0.3,
    is_draft: Math.random() < 0.1,

    // We probably want to determine this from the cards later on
    offline_size: faker.datatype.number({ min: 1, max: 1000 }),
    offline_nsfw_size: faker.datatype.number({ min: 1, max: 200 }),
    online_size: faker.datatype.number({ min: 1, max: 1000 }),
    online_nsfw_size: faker.datatype.number({ min: 1, max: 200 }),

    game_id: game.id,
  };
}

async function main() {
  // generate preset games.
  await prisma.game.create({
    data: {
      title: "Drunk Pirate",
      description: "A drinking game",

      has_image: true,
      is_official: true,
      is_available_online: true,
      is_available_offline: true,

      authors: {
        create: [
          { role: Role.AUTHOR, author: { create: { name: "Owlive", avatar_id: "marceline" } } },
          { role: Role.CONTRIBUTOR, author: { create: { name: "Usyer", avatar_id: "ghost" } } },
        ],
      },

      addons: {
        createMany: {
          data: [
            {
              title: "Base Pack",
              description: "The default set of cards",
              is_official: true,
              has_image: true,
              is_draft: false,
              online_size: 300,
              offline_size: 125,
            },
            {
              title: "Owl pack",
              description: "A set of owl themed cards",
              is_official: false,
              has_image: true,
              is_draft: false,
              online_size: 200,
              offline_size: 25,
            },
          ],
        },
      },
    },
  });

  const game = await prisma.game.create({
    data: {
      title: "Tipsy Sailor",
      description: "gamer gaming game",

      has_image: true,
      is_official: false,
      is_available_online: true,
      is_available_offline: false,

      authors: {
        create: [
          { role: Role.AUTHOR, author: { create: { name: "Julien", avatar_id: "flapjack" } } },
          { role: Role.CONTRIBUTOR, author: { create: { name: "Tetro", avatar_id: "jake" } } },
        ],
      },
    },
  });

  // create addons.
  await prisma.addon.createMany({
    data: createArray(30, 60).map(() => createAddon(game)),
  });

  // create cards.
  const addons = await prisma.addon.findMany();
  for (const addon of addons) {
    const data = createArray(50, 300).map(() => createCard(addon));
    await prisma.$transaction(data);
  }
}

main()
  .then(async () => await prisma.$disconnect())
  .catch(async (e) => {
    console.error(e);

    await prisma.$disconnect();

    process.exit(1);
  });
