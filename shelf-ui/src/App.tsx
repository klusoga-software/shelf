import { AppLayout, TopNavigation } from "@cloudscape-design/components";
import { Route, Routes } from "react-router-dom";
import ReposPage from "./pages/ReposPage.tsx";
import Sidenav from "./components/Sidenav.tsx";
import CratesPage from "./pages/CratesPage.tsx";
import { useContext, useEffect, useState } from "react";
import { hasAuthParams, useAuth } from "react-oidc-context";
import ServiceAccountsPage from "./pages/ServiceAccountsPage.tsx";
import { NotificationContext } from "./components/NotificationProvider.tsx";

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

  const notificationContext = useContext(NotificationContext);
  const { alerts } = notificationContext!;

  return (
    <>
      <TopNavigation identity={{ href: "ui", title: "Shelf" }}></TopNavigation>
      <AppLayout
        contentType={"dashboard"}
        navigation={<Sidenav />}
        notifications={alerts()}
        content={
          <Routes>
            <Route path="/repos" element={<ReposPage />} />
            <Route path="/crates/:id" element={<CratesPage />} />
            <Route path="/service-accounts" element={<ServiceAccountsPage />} />
          </Routes>
        }
      ></AppLayout>
    </>
  );
}

export default App;
