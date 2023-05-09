import { Card, useTheme } from "@geist-ui/core";
import dynamic from "next/dynamic";

const LazyLog = dynamic(
  () => import("react-lazylog").then((mod) => mod.LazyLog),
  {
    ssr: false,
  }
);

const ScrollFollow = dynamic(
  () => import("react-lazylog").then((mod) => mod.ScrollFollow),
  {
    ssr: false,
  }
);

type LogProps = {
  text: string | undefined;
};

export default function LogOutput(props: LogProps) {
  const theme = useTheme();

  let dark = { backgroundColor: "transparent", color: "#FFF" };
  let white = { backgroundColor: "transparent", color: "#000", };

  return (
    <Card>
      <div style={{ height: 400, width: 680 }}>
        <ScrollFollow
          startFollowing
          render={({ follow }) => (
            <LazyLog
              style={theme.type === "dark" ? dark : white}
              url=""
              text={props.text}
              follow={follow}
            />
          )}
        />
      </div>
    </Card>
  );
}
