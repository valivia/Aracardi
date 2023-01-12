import styles from "./setup_item.module.scss";
import { Tag } from "@components/global/tag";
import { faker } from "@faker-js/faker";
import React, { useMemo } from "react";
import { BsWifi, BsWifiOff } from "react-icons/bs";
import { RouterOutput } from "@utils/trpc";

const Addon: React.FC<Props> = ({ addon, active, onClick }) => {
  const avatar = useMemo(() => faker.image.abstract(640, 640, true), []);
  return (
    <article
      className={styles.main}
      data-active={active}
      onClick={onClick}
      tabIndex={0}
    >

      {/* Addon avatar */}
      <img className={styles.avatar} src={avatar} alt="" />

      {/* Title, card count and tags */}
      <section className={styles.info}>

        <h2 className={styles.title}>{addon.title}</h2>
        <p className={styles.description}>{addon.description}</p>

        <section className={styles.tags}>
          <Tag tooltip="Amount of offline cards contained in this addon"><BsWifi />{addon.online_size}</Tag>
          <Tag tooltip="Amount of offline cards contained in this addon"><BsWifiOff />{addon.offline_size}</Tag>
          {addon.is_official && <Tag tooltip="This is a verified addon">Official</Tag>}
        </section>

      </section>

      {/* Selection indicator */}
      {typeof active === "boolean" &&
        <div className={styles.indicator} data-active={active}>
          <div></div>
        </div>
      }
    </article>
  );
};

export { Addon };

interface Props {
  addon: RouterOutput["addon"]["get"];
  onClick?: () => void | Promise<void>;
  active?: boolean;
}
