import styles from "./number.module.scss";

function NumberInputComponent({ text: inputText, name, value, min, max, step, onChange }: Props) {
  const text = inputText ?? name;
  return (
    <fieldset className={styles.main}>
      <label
        className={styles.label}
        htmlFor={name}
      >
        {text}
      </label>
      <input
        min={min}
        max={max}
        step={step}

        id={name}
        type="number"
        name={name}
        onChange={onChange}
        value={value}
        className={styles.input}
      />
    </fieldset>
  );
}

export default NumberInputComponent;

interface Props {
  text?: string;
  name: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void
}
