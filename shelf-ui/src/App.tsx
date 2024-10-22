import { TopNavigation } from "@cloudscape-design/components";
import { Route, Routes } from "react-router-dom";
import ReposPage from "./pages/ReposPage.tsx";
import CratesPage from "./pages/CratesPage.tsx";
import { useEffect, useState } from "react";
import { hasAuthParams, useAuth } from "react-oidc-context";
import ServiceAccountsPage from "./pages/ServiceAccountsPage.tsx";
import Dashboard from "./pages/Dashboard.tsx";

function App() {
  const auth = useAuth();
  const [hasTriedSignin, setHasTriedSignin] = useState(false);

  useEffect(() => {
    if (
      !hasAuthParams() &&
      !auth.isAuthenticated &&
      !auth.activeNavigator &&
      !auth.isLoading &&
      !hasTriedSignin
    ) {
      auth.signinRedirect().then(() => {});
      setHasTriedSignin(true);
    }

    console.log(auth.user);
  }, [auth, hasTriedSignin]);

  return (
    <>
      <TopNavigation identity={{ href: "/", title: "Shelf" }}></TopNavigation>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/repos" element={<ReposPage />} />
        <Route path="/crates/:id" element={<CratesPage />} />
        <Route path="/service-accounts" element={<ServiceAccountsPage />} />
      </Routes>
    </>
  );
}

export default App;
