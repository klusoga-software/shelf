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
      <TopNavigation
        identity={{ href: "/", title: "Shelf" }}
        utilities={[
          {
            type: "menu-dropdown",
            text: auth.user?.profile.name,
            onItemClick: async ({ detail }) => {
              switch (detail.id) {
                case "signout":
                  await auth.signoutSilent();
              }
            },
            description: auth.user?.profile.email,
            iconName: "user-profile",
            items: [
              {
                id: "support-group",
                text: "Support",
                items: [
                  {
                    id: "documentation",
                    text: "Documentation",
                    href: "https://klusoga-software.github.io/shelf/",
                    external: true,
                    externalIconAriaLabel: " (opens in new tab)",
                  },
                  {
                    id: "feedback",
                    text: "Feedback",
                    href: "https://github.com/klusoga-software/shelf/issues",
                    external: true,
                    externalIconAriaLabel: " (opens in new tab)",
                  },
                ],
              },
              { id: "signout", text: "Sign out" },
            ],
          },
        ]}
      ></TopNavigation>
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
