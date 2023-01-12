import styles from "./index.module.scss";
import { Layout } from "src/components/global/layout";
import { TextInput } from "@components/input/text_input";
import { Button } from "@components/input/button";
import { useState } from "react";
import { Link } from "@components/input/link";
import { NextPage } from "next";

const Home: NextPage = () => {
  const [joinCode, setJoinCode] = useState("");

  const fetchSession = async () => {
    // TODO make this do something
  };

  return (
    <Layout title="Aracardi">
      <main className={styles.main}>

        <h1 className={styles.title}>Play now!</h1>
        <div className={styles.divider} />

        <form className={styles.form}>

          {/* Join game */}
          <fieldset className={styles.joinGame}>
            <TextInput
              value={joinCode}
              onChange={(e) => setJoinCode(e.target.value)}

              size="lg"
              placeholder="Join Code"
            />
            <Button
              onClick={fetchSession}
              size="lg"
              variant="secondary"
            >
              Join Game
            </Button>
          </fieldset>

          <div className={styles.divider} />

          {/* Create game */}
          <Link
            size="lg"
            href="/game"
          >
            Create Game
          </Link>
        </form>

      </main>
    </Layout>
  );
};

export default Home;
