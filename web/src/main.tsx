import InitWasm from "@/components/providers/init-wasm.tsx";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";

// biome-ignore lint:
ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <InitWasm>
      <App />
    </InitWasm>
  </React.StrictMode>,
);
