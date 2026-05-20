import "./main.css";
/* @refresh reload */
import { render } from "solid-js/web";
import { RouterProvider, createRouter } from "@tanstack/solid-router";

// Import the generated route tree
import { routeTree } from "./routeTree.gen";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools";

const queryClient = new QueryClient();

// Create a new router instance
const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/solid-router" {
  interface Register {
    router: typeof router;
  }
}

// Render the app
const rootElement = document.getElementById("root")!;

render(
  () => (
    <>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
        <SolidQueryDevtools initialIsOpen={false} />
      </QueryClientProvider>
    </>
  ),
  rootElement,
);
