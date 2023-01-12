import styles from "@styles/dashboard.module.scss";
import { Layout } from "src/components/global/layout";
import { Header } from "@components/dashboard/header";
import { Accordion } from "@components/dashboard/accordion";
import Prisma, { Role } from "@prisma/client";
import { trpc } from "@utils/trpc";
import { GetStaticPaths, GetStaticProps, NextPage } from "next";
import { prisma } from "src/server/prisma";
import { Button } from "@components/input/button";
import { Toggle } from "@components/input/toggle";
import { User } from "@components/dashboard/user";

import { useBoolean } from "@hooks/useBoolean";
import { Avatar } from "@components/global/avatar";
import { DashboardItem } from "@components/dashboard/dashboard_item";
import { Tag } from "@components/global/tag";
import { BsWifi, BsWifiOff } from "react-icons/bs";

const GameDashboard: NextPage<Props> = ({ game }) => {
  const addons = trpc.addon.all.useQuery({ limit: 5, game_id: game.id });
  const { value: allowNsfw, toggle: toggleAllowNsfw } = useBoolean(true);

  return (
    <Layout>
      <div className={styles.main}>
        <Header
          title={game.title}
          description={game.description}
          avatar={<Avatar id="ghost" />}
        />

        <main className={styles.menu}>
          <Accordion title="Statistics" defaultExpanded={true}>
            a
          </Accordion>

          <Accordion title="Default Addons" defaultExpanded={true}>
            <section className={styles.itemList}>

              {addons.data?.items.map(addon =>
                <DashboardItem
                  key={addon.id}
                  title={addon.title}
                  href={`/dashboard/addon/${addon.id}`}
                  avatar={<Avatar id="ghost" />}
                >
                  <Tag tooltip="Amount of offline cards contained in this addon"><BsWifi />{addon.online_size}</Tag>
                  <Tag tooltip="Amount of offline cards contained in this addon"><BsWifiOff />{addon.offline_size}</Tag>
                  {addon.is_official && <Tag tooltip="This is a verified addon">Official</Tag>}
                </DashboardItem>
              )}

              <Button variant="secondary">Add default addon</Button>
            </section>
          </Accordion>

          <Accordion title="Default Settings">
            <section className={styles.settings}>
              <Toggle
                name="allow_nsfw"
                label="Allow nsfw"
                value={allowNsfw}
                disabled={true}
                onChange={toggleAllowNsfw}
              />
              <Toggle
                name="loop_cards"
                label="Loop cards"
                value={allowNsfw}
                onChange={toggleAllowNsfw}
              />

              <Toggle
                name="available_online"
                label="Available online"
                value={allowNsfw}
                onChange={toggleAllowNsfw}
              />

              <Toggle
                name="available_offline"
                label="Available offline"
                value={allowNsfw}
                onChange={toggleAllowNsfw}
              />
            </section>
          </Accordion>

          <Accordion title="Permissions">
            <section className={styles.permissions}>
              <User user={{ name: "Owlive", avatar_id: "marceline" }} role={Role.AUTHOR} />
              <User user={{ name: "Usyer", avatar_id: "ghost" }} canEdit role={Role.CONTRIBUTOR} />
              <User user={{ name: "Birbreme", avatar_id: "froggi" }} canEdit role={Role.CONTRIBUTOR} />
              <Button variant="secondary">Add collaborator</Button>
            </section>
          </Accordion>
        </main>

      </div>
    </Layout>
  );
};

interface Props {
  game: Prisma.Game;
}

export const getStaticPaths: GetStaticPaths = async () => {
  const result = await prisma.game.findMany();
  const paths = result.map((project) => ({ params: { id: project.id } }));
  return {
    paths,
    fallback: "blocking",
  };
};

export const getStaticProps: GetStaticProps = async ({ params }) => {
  const game = await prisma.game.findUnique({ where: { id: params?.id as string } });
  if (!game) return { notFound: true };

  return { props: { game } };
};

export default GameDashboard;
