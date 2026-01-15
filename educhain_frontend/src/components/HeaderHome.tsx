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
                <Button variant="outline" className="rounded-xl p-2 w-50 font-semibold">Iniciar sesión</Button>
                <Button variant={"outline"} className="rounded-xl p-2 w-50 font-semibold">Crear cuenta</Button>
            </div>
        </nav>
    )
}
