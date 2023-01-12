import { useRouter } from "next/router";
import styles from "./logo.module.scss";

function LogoComponent() {
  const router = useRouter();
  return (
    <div className={styles.main}>
      <svg
        onClick={() => confirm("Are you sure you want to leave?") && router.replace("/")}
        viewBox="0 0 145.72 71"
      >
        <g>
          <polygon points="110.86 10.5 76.86 70.5 144.86 70.5 110.86 10.5" />
          <polygon points="110.86 20.1 85.46 64.9 136.26 64.9 110.86 20.1" />
        </g>
        <g>
          <polygon points="72.86 60.5 106.86 .5 38.86 .5 72.86 60.5" />
        </g>
        <g>
          <polygon points="34.86 10.5 .86 70.5 68.86 70.5 34.86 10.5" />
        </g>
      </svg>
    </div>
  );
}

export default LogoComponent;
