import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

export const RegisterPage = () => {
  return (
    <div className="min-h-screen flex items-center justify-center bg-primary">
      <div className="flex flex-col text-center p-10 w-2xl h-fit bg-secondary rounded">
        <h2 className="text-5xl font-bold mb-10">Sign up!</h2>
        <form className="flex flex-col items-center gap-5">
          <div className="w-full flex flex-col gap-5">
            <Label className="font-semibold" htmlFor="email">Email</Label>
            <Input type="email" />
          </div>
          <div className="w-full flex flex-col gap-5">
            <Label className="font-semibold" htmlFor="email">Username</Label>
            <Input type="text" />
          </div> 
          <div className="w-full flex flex-col gap-5">
            <Label className="font-semibold" htmlFor="password">Password</Label>
            <Input type="password" />
          </div>
          <div className="w-full flex flex-col gap-5">
            <Label className="font-semibold" htmlFor="password">Repeat password</Label>
            <Input type="password" />
          </div>          
          <Button variant={"default"} className="w-full">
            Log in
          </Button>
        </form>
      </div>
    </div>
  )
}
