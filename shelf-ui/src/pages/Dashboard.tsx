import {
  AppLayout,
  Box,
  Button,
  ContentLayout,
  Header,
  SpaceBetween,
} from "@cloudscape-design/components";
import {
  Board,
  BoardItem,
  BoardProps,
} from "@cloudscape-design/board-components";
import { useContext, useEffect, useState } from "react";
import { DashboardItemData } from "../models/dashboard-item-data.ts";
import Sidenav from "../components/Sidenav.tsx";
import { NotificationContext } from "../components/NotificationProvider.tsx";
import CountWidget from "../components/widgets/CountWidget.tsx";
import axios from "axios";
import { useAuth } from "react-oidc-context";
import { DashboardData } from "../models/dashboard-data.ts";

function Dashboard() {
  const [dashboardData, setDashboardData] = useState<DashboardData>({
    repoCount: 0,
    storage: 0,
  });

  const [items, setItems] = useState<
    readonly BoardProps.Item<DashboardItemData>[]
  >([
    {
      id: "count",
      data: {
        type: "count",
        header: "Repo Count",
      },
    },
    {
      id: "storage",
      data: {
        type: "storage",
        header: "Total Storage",
      },
    },
  ]);

  const notificationContext = useContext(NotificationContext);
  const { showNotification, alerts } = notificationContext!;
  const auth = useAuth();

  useEffect(() => {
    if (auth.isAuthenticated) {
      getDashboardData();
    }
  }, [auth]);

  useEffect(() => {
    setItems((prevState) => prevState);
  }, [dashboardData]);

  function getDashboardData() {
    axios
      .get("/api/dashboard/data")
      .then((response) => {
        setDashboardData(response.data);
      })
      .catch((err) => {
        showNotification({
          type: "error",
          header: "Error while fetching dashboard data",
          message: err.response?.data,
        });
      });
  }

  function save() {
    const dashboardData = items.map((item) => {
      return {
        id: item.id,
        rowSpan: item.rowSpan,
        columnSpan: item.columnSpan,
        columnOffset: item.columnOffset,
      };
    });

    const request = { tiles: dashboardData };

    axios
      .post("/api/dashboard", request, {
        headers: { Authorization: `Bearer ${auth.user?.access_token}` },
      })
      .then(() => {})
      .catch((err) => {
        showNotification({
          type: "error",
          header: "Error while delete repo",
          message: err.response?.data,
        });
      });
  }

  function widgetSwitch(item: BoardProps.Item<DashboardItemData>) {
    switch (item.data.type) {
      case "count":
        return <CountWidget count={dashboardData.repoCount} />;
      case "storage":
        return <CountWidget count={dashboardData.storage} />;
    }
  }

  return (
    <AppLayout
      contentType="dashboard"
      navigation={<Sidenav active="/" />}
      notifications={alerts()}
      toolsHide={true}
      content={
        <ContentLayout
          header={
            <Header
              variant="h1"
              actions={
                <SpaceBetween size="s">
                  <Button onClick={save}>Save</Button>
                </SpaceBetween>
              }
            >
              Dashboard
            </Header>
          }
        >
          <Board
            items={items}
            renderItem={(item) => (
              <BoardItem
                header={<Header>{item.data.header}</Header>}
                i18nStrings={{
                  dragHandleAriaLabel: "Drag Handle",
                  resizeHandleAriaLabel: "Resize Handle",
                }}
              >
                {widgetSwitch(item)}
              </BoardItem>
            )}
            //@ts-expect-error temp hack
            i18nStrings={{
              liveAnnouncementDndCommitted: () => "",
              liveAnnouncementDndDiscarded: () => "",
              liveAnnouncementDndItemInserted: () => "",
              liveAnnouncementDndItemReordered: () => "",
              liveAnnouncementDndItemResized: () => "",
              liveAnnouncementDndStarted: () => "",
              liveAnnouncementItemRemoved: () => "",
            }}
            onItemsChange={(event) => {
              setItems(event.detail.items);
            }}
            empty={
              <Box
                margin={{ vertical: "xs" }}
                textAlign="center"
                color="inherit"
              >
                <SpaceBetween size="m">
                  <Box variant="strong" color="inherit">
                    No items
                  </Box>
                  <Button iconName="add-plus">Add an item</Button>
                </SpaceBetween>
              </Box>
            }
          ></Board>
        </ContentLayout>
      }
    ></AppLayout>
  );
}

export default Dashboard;
