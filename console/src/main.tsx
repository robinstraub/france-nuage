import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { Provider } from "./components/chakra/provider";
import { ReactReduxProvider } from "./providers/redux-provider";

const root = document.getElementById("root");

if (root) {
  createRoot(root).render(
    <StrictMode>
      <ReactReduxProvider>
        <Provider>
          <App />
        </Provider>
      </ReactReduxProvider>
    </StrictMode>,
  );
}
