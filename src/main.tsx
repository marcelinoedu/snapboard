import ReactDOM from "react-dom/client";
import App from "./App";
import React from "react";
import { OverlayProvider } from "./contexts/overlayContext";
import "./styles/App.css";
import "./styles/tailwind.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <OverlayProvider>
      <App />
    </OverlayProvider>
  </React.StrictMode>
);