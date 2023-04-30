import dynamic from "next/dynamic";

const DynamicRestore = dynamic(() => import("../components/DynamicRestore"), {
  ssr: false,
});

export default function Home() {
  return <DynamicRestore />;
}
