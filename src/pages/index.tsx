import dynamic from 'next/dynamic'

const DynamicIndex = dynamic(() => import('../components/DynamicIndex'), {
  ssr: false,
})

export default function Home() {
  return <DynamicIndex />
}