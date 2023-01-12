import styles from "./setup_item.module.scss";
import { Tag } from "@components/global/tag";
import { faker } from "@faker-js/faker";
import prisma from "@prisma/client";
import React, { useMemo } from "react";
import { BsWifi, BsWifiOff } from "react-icons/bs";

// TODO display correct card count. keyboard accessibility (keydown and infinite load?)

const Game: React.FC<Props> = ({ game, active, onClick }) => {
  const avatar = useMemo(() => faker.image.abstract(640, 640, true), []);
  return (
    <article
      className={styles.main}
      onClick={onClick}
      tabIndex={0}
      data-active={active}
    >

      {/* Game avatar */}
      <img className={styles.avatar} src={avatar} alt="" />

      {/* Title, description,  */}
      <section className={styles.info}>

        <h2 className={styles.title}>{game.title}</h2>
        <p className={styles.description}>{game.description}</p>

        <section className={styles.tags}>
          <Tag><BsWifi /></Tag>
          <Tag><BsWifiOff /></Tag>
          {game.is_official && <Tag>Official</Tag>}
        </section>

      </section>
    </article>
  );
};

export { Game };

interface Props {
  game: prisma.Game;
  onClick?: () => void | Promise<void>;
  active?: boolean;
}
