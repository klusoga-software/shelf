import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "@cloudscape-design/global-styles/index.css";
import {BrowserRouter} from "react-router-dom";
import axios from "axios";

if (import.meta.env.MODE === 'development'){
    axios.defaults.baseURL = "http://localhost:6300/";
}


createRoot(document.getElementById("root")!).render(
  <StrictMode>
      <BrowserRouter>
          <App />
      </BrowserRouter>
  </StrictMode>,
);
