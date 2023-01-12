import Link from "next/link";
import React, { PropsWithChildren, ReactNode } from "react";
import { BsGear } from "react-icons/bs";
import styles from "./dashboard_item.module.scss";

// TODO settings?

const DashboardItem: React.FC<PropsWithChildren<Props>> = ({ title, href, avatar, children }) => {
  return (
    <Link
      href={href}
      className={styles.main}
      tabIndex={0}
    >

      {/* Avatar */}
      <figure className={styles.avatar}>{avatar}</figure>

      {/* Info */}
      <section className={styles.info}>
        <h2 className={styles.title}>{title}</h2>

        {/* Tags */}
        <section className={styles.tags}>
          {children}
        </section>

      </section>

      {/* Settings */}
      <button
        className={styles.settings}
        onClick={(e) => { e.stopPropagation(); e.nativeEvent.preventDefault(); }}
      >
        <BsGear />
      </button>
    </Link>
  );
};

export { DashboardItem };

interface Props {
  title: string;
  href: string;
  avatar: ReactNode;
}
