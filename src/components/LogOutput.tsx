import { Card } from "@geist-ui/core";
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
  return (
    <Card>
      <div style={{ height: 400, width: 680 }}>
        <ScrollFollow
          startFollowing
          render={({ follow }) => (
            <LazyLog
              style={{ backgroundColor: "#000", color: "#FAFAFA" }}
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
