import {
  AppLayout,
  Box,
  Button,
  ContentLayout,
  Header,
  SpaceBetween,
  Spinner,
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
import { DashboardResponse } from "../models/dashboard-response.ts";
import StorageWidget from "../components/widgets/StorageWidget.tsx";

function Dashboard() {
  const [dashboardData, setDashboardData] = useState<DashboardData>({
    repoCount: 0,
    storage: 0,
  });

  const [items, setItems] = useState<
    readonly BoardProps.Item<DashboardItemData>[]
  >([]);

  const DEFAULT_WIDGETS: readonly BoardProps.Item<DashboardItemData>[] = [
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
  ];

  const notificationContext = useContext(NotificationContext);
  const { showNotification, alerts } = notificationContext!;
  const auth = useAuth();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (auth.isAuthenticated) {
      getDashboard();
      getDashboardData();
    }
  }, [auth]);

  useEffect(() => {
    setItems((prevState) => prevState);
  }, [dashboardData]);

  function getDashboard() {
    axios
      .get<DashboardResponse[]>("/api/dashboard", {
        headers: { Authorization: `Bearer ${auth.user?.access_token}` },
      })
      .then((response) => {
        setItems([]);
        for (const tile of response.data) {
          switch (tile.id) {
            case "count":
              setItems((prevState) => [
                ...prevState,
                {
                  id: tile.id,
                  data: {
                    type: "count",
                    header: "Repo Count",
                  },
                  rowSpan: tile.rowSpan,
                  columnSpan: tile.columnSpan,
                  columnOffset: tile.columnOffset,
                },
              ]);
              break;
            case "storage":
              setItems((prevState) => [
                ...prevState,
                {
                  id: tile.id,
                  data: {
                    type: "storage",
                    header: "Total Storage",
                  },
                  rowSpan: tile.rowSpan,
                  columnSpan: tile.columnSpan,
                  columnOffset: tile.columnOffset,
                },
              ]);
          }
        }
      })
      .catch((err) => {
        if (err.status === 404) {
          setItems(DEFAULT_WIDGETS);
          return;
        }
        showNotification({
          type: "error",
          header: "Error while fetching dashboard data",
          message: err.response?.data,
        });
      });
  }

  function getDashboardData() {
    axios
      .get("/api/dashboard/data", {
        headers: { Authorization: `Bearer ${auth.user?.access_token}` },
      })
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
    setLoading(true);
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
      .then(() => {
        showNotification({
          type: "success",
          header: "Dashboard saved",
          message: "Dashboard saved successfully",
        });
        setLoading(false);
      })
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
        return <CountWidget link="repos" count={dashboardData.repoCount} />;
      case "storage":
        return <StorageWidget link="repos" count={dashboardData.storage} />;
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
                  <Button loading={loading} onClick={save}>
                    Save
                  </Button>
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
                  <Spinner size="large" />
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
