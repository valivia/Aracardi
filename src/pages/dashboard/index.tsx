import styles from "@styles/dashboard.module.scss";
import { Layout } from "src/components/global/layout";
import { Header } from "@components/dashboard/header";
import { Accordion } from "@components/dashboard/accordion";
import { User } from "@prisma/client";
import { trpc } from "@utils/trpc";
import { Avatar } from "@components/global/avatar";
import { DashboardItem } from "@components/dashboard/dashboard_item";
import { BsWifi, BsWifiOff } from "react-icons/bs";
import { Tag } from "@components/global/tag";
import { NextPage } from "next";


const Dashboard: NextPage = () => {
  const user: User = {
    name: "Owlive",
    created_at: new Date(),
    updated_at: new Date(),
    id: "",
    avatar_id: "marceline",
  };

  // TODO: proper query for user's games.
  const games = trpc.game.all.useQuery({ limit: 5 });

  return (
    <Layout>
      <div className={styles.main}>
        <Header
          title={user.name}
          avatar={<Avatar id={user.avatar_id || "pb"} />}
        />

        <main className={styles.menu}>
          <Accordion title="My Games" defaultExpanded={true}>
            <section className={styles.itemList}>

              {games.data?.items.map(game =>
                <DashboardItem
                  key={game.id}
                  title={game.title}
                  href={`/dashboard/game/${game.id}`}
                  avatar={<Avatar id="froggi" />}
                >
                  {game.is_available_online && <Tag tooltip="Is available online"><BsWifi /></Tag>}
                  {game.is_available_offline && <Tag tooltip="Is available offline"><BsWifiOff /></Tag>}
                  {game.is_official && <Tag tooltip="This is a verified addon">Official</Tag>}
                </DashboardItem>
              )}

            </section>
          </Accordion>

          <Accordion title="My Addons" defaultExpanded={true}>
            <span>aaa</span>
          </Accordion>
        </main>

      </div>
    </Layout>
  );
};

export default Dashboard;
