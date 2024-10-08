import { SideNavigation } from "@cloudscape-design/components";
import { useNavigate } from "react-router-dom";
import React from "react";

function Sidenav() {
  const [active, setActive] = React.useState("/");
  const navigate = useNavigate();

  function onFollow(href: string) {
    navigate(href);
  }

  return (
    <SideNavigation
      activeHref={active}
      onFollow={(event) => {
        if (!event.detail.external) {
          event.preventDefault();
          setActive(event.detail.href);
          onFollow(event.detail.href);
        }
      }}
      items={[
        { type: "link", text: "Dashboard", href: "/" },
        { type: "link", text: "Repos", href: "/repos" },
        { type: "link", text: "Service Accounts", href: "/service-accounts" },
      ]}
    />
  );
}

export default Sidenav;
