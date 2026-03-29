import type { AppProps } from "next/app";
import { useEffect } from "react";
import { Noto_Sans_JP } from "next/font/google";
import { ThemeProvider } from "@/context/ThemeProvider";
import { AlertsProvider } from "@/context/AlertsProvider";
import "@/styles/globals.scss";

const notoSansJP = Noto_Sans_JP({
  subsets: ["latin"],
  weight: ["400", "700"],
  display: "swap",
});

export default function App({ Component, pageProps }: AppProps) {
  useEffect(() => {
    import("bootstrap");
  }, []);

  return (
    <ThemeProvider>
      <AlertsProvider>
        <div className={notoSansJP.className}>
          <Component {...pageProps} />
        </div>
      </AlertsProvider>
    </ThemeProvider>
  );
}
