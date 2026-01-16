import { Button } from "@/components/ui/button"
import { Bot } from "lucide-react"
import { Link } from "react-router"

export const NotFound404 = () => {
  return (
    <div className="min-h-screen flex flex-col bg-accent-foreground justify-center items-center">
      <div className="w-2xl">
        <Bot size={100} className="mb-5 text-secondary" />
        <h2 className="text-8xl text-secondary">Sorry... nothing to see here.</h2>
        <Button variant={"outline"} className="mt-10 bg-secondary text-primary font-semibold w-full rounded">
          <Link to={"/"}>
            Go back
          </Link>
          </Button>
      </div>
    </div>
  )
}
