import { Bot } from "lucide-react"

export const NotFound404 = () => {
  return (
    <div className="min-h-screen flex flex-col bg-primary justify-center items-center">
        <Bot size={100} className="mb-5 text-secondary" />
        <h2 className="text-8xl text-secondary">Sorry... nothing to see here.</h2>
    </div>
  )
}
