import { motion } from "framer-motion";
import Head from "next/head";
import styles from "./layout.module.scss";
import LogoComponent from "./logo";

function Layout({ title, subtitle, description, children }: Props) {
  return (
    <div className={styles.frame}>
      <Head>
        <title>{title}</title>
        {description && <meta name="description" content={description} />}
      </Head>

      {(title || subtitle) &&
        <motion.header
          key={title || subtitle}
          className={styles.title}
          initial={{ x: 300 }}
          animate={{ x: 0 }}
        >
          {title && <h1>{title}</h1>}
          {subtitle && <p>{subtitle}</p>}
        </motion.header>
      }
      {children}

      <LogoComponent />
    </div>
  );
}

export { Layout };

interface Props {
  title?: string;
  subtitle?: string;
  description?: string;
  children: JSX.Element | JSX.Element[];
}
