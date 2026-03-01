import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { AppLayout } from "./components/app-layout";
import { PageGuard } from "./components/page-guard";
import { CallbackPage } from "./pages/callback.page";
import { HomePage } from "./pages/home.page";

const router = createBrowserRouter([
  {
    path: "/callback",
    element: <CallbackPage />,
  },
  {
    element: <PageGuard />,
    children: [
      {
        element: <AppLayout />,
        children: [
          {
            index: true,
            element: <HomePage />,
          },
        ],
      },
    ],
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
