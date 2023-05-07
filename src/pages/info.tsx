import dynamic from "next/dynamic";

const DynamicInfo = dynamic(() => import("../components/DynamicInfo"), {
  ssr: false,
});

export default function Home() {
  return <DynamicInfo />;
}
