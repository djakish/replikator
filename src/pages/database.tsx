import dynamic from "next/dynamic";

const DynamicDatabase = dynamic(() => import("../components/DynamicDatabase"), {
  ssr: false,
});

export default function Home() {
  return <DynamicDatabase />;
}
