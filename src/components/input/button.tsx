import { ElementSize } from "@structs/size";
import React, { PropsWithChildren } from "react";
import styles from "./button.module.scss";


const Button: React.FC<PropsWithChildren<Props>> = ({
  children,
  size = "md",
  variant = "primary",
  type = "button",
  onClick = () => undefined,
}) => {

  return (
    <button
      className={styles.main}

      type={type}
      data-variant={variant}
      data-size={size}

      onClick={onClick}
    >
      {children}
    </button>
  );
};

export { Button };

interface Props {
  variant?: "primary" | "secondary";
  type?: "button" | "submit" | "reset";
  size?: ElementSize;
  onClick?: () => void | Promise<void>;
}
