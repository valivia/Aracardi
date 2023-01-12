import styles from "@styles/dashboard.module.scss";
import { Layout } from "src/components/global/layout";
import { Header } from "@components/dashboard/header";
import { Accordion } from "@components/dashboard/accordion";
import Prisma, { Role } from "@prisma/client";
import { GetStaticPaths, GetStaticProps, NextPage } from "next";
import { prisma } from "src/server/prisma";
import { Button } from "@components/input/button";
import { Toggle } from "@components/input/toggle";
import { User } from "@components/dashboard/user";

import { useBoolean } from "@hooks/useBoolean";
import { Avatar } from "@components/global/avatar";

const AddonDashboard: NextPage<Props> = ({ addon }) => {
  const { value: allowNsfw, toggle: toggleAllowNsfw } = useBoolean(true);

  return (
    <Layout>
      <div className={styles.main}>
        <Header
          title={addon.title}
          description={addon.description}
          avatar={<Avatar id="ghost" />}
        />

        <main className={styles.menu}>
          <Accordion title="Statistics" defaultExpanded={true}>
            a
          </Accordion>

          <Accordion title="Default Settings">
            <section className={styles.settings}>
              <Toggle
                name="allow_nsfw"
                label="Allow nsfw"
                value={allowNsfw}
                disabled={true}
                onChange={() => toggleAllowNsfw()}
              />
              <Toggle
                name="loop_cards"
                label="Loop cards"
                value={allowNsfw}
                onChange={() => toggleAllowNsfw()}
              />

              <Toggle
                name="available_online"
                label="Available online"
                value={allowNsfw}
                onChange={() => toggleAllowNsfw()}
              />

              <Toggle
                name="available_offline"
                label="Available offline"
                value={allowNsfw}
                onChange={() => toggleAllowNsfw()}
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
  addon: Prisma.Addon;
}

export const getStaticPaths: GetStaticPaths = async () => {
  const result = await prisma.addon.findMany();
  const paths = result.map((addon) => ({ params: { addon_id: addon.id } }));
  return {
    paths,
    fallback: "blocking",
  };
};

export const getStaticProps: GetStaticProps = async ({ params }) => {
  const addon = await prisma.addon.findUnique({ where: { id: params?.addon_id as string } });
  if (!addon) return { notFound: true };

  return { props: { addon } };
};

export default AddonDashboard;
