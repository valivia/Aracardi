import React, { PropsWithChildren, useState } from "react";
import styles from "./accordion.module.scss";

const Accordion: React.FC<PropsWithChildren<Props>> = ({ children, defaultExpanded, title }) => {
  const [expanded, setExpanded] = useState(defaultExpanded || false);

  return (
    <section className={styles.main}>

      <button
        className={styles.bar}
        aria-expanded={expanded}
        onClick={() => setExpanded(old => !old)}
      >
        <span className={styles.title}>{title}</span>
        <svg className={styles.icon} aria-hidden="true" viewBox="0 0 24 24">
          <path d="M16.59 8.59 12 13.17 7.41 8.59 6 10l6 6 6-6z"></path>
        </svg>
      </button>

      {expanded &&
        <div className={styles.content}>
          {children}
        </div>
      }

    </section>
  );
};

export { Accordion };

interface Props {
  title: string;
  defaultExpanded?: boolean;
}
