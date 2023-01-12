import "../styles/globals.scss";
import type { AppProps } from "next/app";
import Head from "next/head";

import { Raleway } from "@next/font/google";
import { trpc } from "@utils/trpc";
const raleway = Raleway({ subsets: ["latin"] });

function App({ Component, pageProps }: AppProps) {
  return <>
    <Head>
      <title>Aracardi</title>
      <meta name="description" content="Play card games!" />
      <link rel="Icon" href="/favicon.ico" />
      <meta name="viewport" content="initial-scale=1.0, width=device-width"></meta>
    </Head>
    <Component className={raleway.className} {...pageProps} />
  </>;
}

export default trpc.withTRPC(App);
