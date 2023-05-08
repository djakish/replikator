import "@/styles/globals.css";
import type { AppProps } from "next/app";
import { GeistProvider, CssBaseline } from "@geist-ui/core";
import { useCallback, useEffect, useState } from "react";
import { PrefersContext, ThemeType, themes } from "@/lib/use-prefers";
import dynamic from "next/dynamic";

const TimeManager = dynamic(() => import("../components/TimeManager"), {
  ssr: false,
});


export default function App({ Component, pageProps }: AppProps) {
  const [themeType, setThemeType] = useState<ThemeType>("dark");

  useEffect(() => {
    document.documentElement.removeAttribute("style");
    document.body.removeAttribute("style");

    const theme = window.localStorage.getItem("theme") as ThemeType;
    if (themes.includes(theme)) setThemeType(theme);
  }, []);

  const switchTheme = useCallback((theme: ThemeType) => {
    setThemeType(theme);
    if (typeof window !== "undefined" && window.localStorage)
      window.localStorage.setItem("theme", theme);
  }, []);

  return (
    <GeistProvider themeType={themeType}>
      <CssBaseline />
      <PrefersContext.Provider value={{ themeType, switchTheme }}>
        <TimeManager />
        <Component {...pageProps} />
      </PrefersContext.Provider>
    </GeistProvider>
  );
}
