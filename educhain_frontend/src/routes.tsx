import { createBrowserRouter } from "react-router";
import { HomePage, LoginPage, RegisterPage } from "./pages";
import { NotFound404 } from "./pages/NotFound404";
import { Layout } from "./components";

export const router = createBrowserRouter([
    {
        path: '/',
        element: <HomePage />
    },
    {
        path: 'login',
        element: <LoginPage/>
    },
    {
        path: 'register',
        element: <RegisterPage/>
    },
    {
        path: 'dashboard',
        children: [
            {index: true, Component: Layout}
        ]
    },
    {
        path: '*',
        element: <NotFound404 />
    }

])