import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "@cloudscape-design/global-styles/index.css";
import {BrowserRouter} from "react-router-dom";
import axios from "axios";
import {AuthProvider, AuthProviderProps} from "react-oidc-context";
import {WebStorageStateStore} from "oidc-client-ts";

if (import.meta.env.MODE === 'development'){
    axios.defaults.baseURL = "http://localhost:6300/";
}

axios.get('/api/configuration').then((res) => {
    const authProviderProps: AuthProviderProps ={
        authority: res.data.authority,
            client_id: "shelf",
        redirect_uri: document.location.origin + "/",
        userStore: new WebStorageStateStore({ store: window.localStorage }),
        scope: "openid email",
    }

    createRoot(document.getElementById("root")!).render(
        <StrictMode>
            <AuthProvider {...authProviderProps}>
                <BrowserRouter>
                    <App />
                </BrowserRouter>
            </AuthProvider>
        </StrictMode>,
    );
})
