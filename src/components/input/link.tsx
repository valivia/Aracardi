import React, { PropsWithChildren } from "react";
import styles from "./button.module.scss";
import NextLink from "next/link";
import { ElementSize } from "@structs/size";


const Link: React.FC<PropsWithChildren<Props>> = ({
  href,
  children,
  size = "md",
  variant = "primary",
}) => {

  return (
    <NextLink
      href={href}
      className={styles.main}

      data-size={size}
      data-variant={variant}
    >
      {children}
    </NextLink>
  );
};

export { Link };

interface Props {
  href: string;
  variant?: "primary" | "secondary";
  size?: ElementSize;
}
