import type { PropsWithChildren } from "react"
import { AppSidebar } from "./Sidebar"
import { SidebarProvider } from "./ui/sidebar"

export const Layout = ({children}: PropsWithChildren) => {
  return (
    <div>
      <SidebarProvider>
        <AppSidebar />
        <main>
          {children}
        </main>
      </SidebarProvider>
    </div>
  )
}
