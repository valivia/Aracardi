import { ElementSize } from "@structs/size";
import React, { HTMLInputTypeAttribute } from "react";
import styles from "./text_input.module.scss";


const TextInput: React.FC<Props> = ({
  placeholder,
  value,
  onChange,
  size = "md",
  type = "text",
}) => {

  return (
    <input
      type={type}
      className={styles.main}
      placeholder={placeholder}

      value={value}
      onChange={onChange}

      data-size={size}
    />
  );
};

export { TextInput };

interface Props {
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;

  type?: HTMLInputTypeAttribute;
  size?: ElementSize;

  placeholder: string;
}
