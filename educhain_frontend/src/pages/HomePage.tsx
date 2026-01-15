import { HeaderHome } from "@/components/HeaderHome"

export const HomePage = () => {
  return (
    <>
      <div className="min-h-screen bg-secondary">
        <HeaderHome />
        <div className="min-h-screen flex flex-col items-center justify-center">
          <div className="flex flex-col items-center justify-center gap-5">
            <h1 className="text-7xl font-bold">Welcome to Educhain</h1>
            <p className="text-2xl">
              Management system for certifications <span className="font-semibold">on-chain in Polkadot</span>!
            </p>
          </div>
        </div>
      </div>
    </>
  )
}
