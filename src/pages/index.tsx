import dynamic from 'next/dynamic'

const DynamicHeader = dynamic(() => import('../components/NoSsrHome'), {
  ssr: false,
})

export default function Home() {
  return <DynamicHeader />
}