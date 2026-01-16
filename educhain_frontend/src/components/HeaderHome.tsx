import { Link } from "react-router"
import { Button } from "./ui/button"

export const HeaderHome = () => {
    return (
        <nav className="flex min-w-full min-h-18 bg-accent-foreground items-center justify-between border-border">
            <div className="mx-5">
                <h1 className="text-2xl font-bold text-secondary">
                    Educhain
                </h1>
            </div>
            <div className="flex gap-5 items-center font-semibold mx-5 text-center">
                <Button variant="outline" className="rounded p-2 w-50 font-semibold">
                    <Link to={"/login"}>
                        Sign In
                    </Link>
                </Button>
                <Button variant="outline" className="rounded p-2 w-50 font-semibold">
                    <Link to={"/register"}>
                        Sign up
                    </Link>
                </Button>
            </div>
        </nav>
    )
}
