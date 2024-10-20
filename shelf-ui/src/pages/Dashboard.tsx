import {
  AppLayout,
  Box,
  Button,
  ButtonDropdown,
  ContentLayout,
  Header,
  SpaceBetween,
  SplitPanel,
} from "@cloudscape-design/components";
import {
  Board,
  BoardItem,
  BoardProps,
} from "@cloudscape-design/board-components";
import { useContext, useState } from "react";
import { DashboardItemData } from "../models/dashboard-item-data.ts";
import Sidenav from "../components/Sidenav.tsx";
import { NotificationContext } from "../components/NotificationProvider.tsx";

function Dashboard() {
  const [items, setItems] = useState<
    readonly BoardProps.Item<DashboardItemData>[]
  >([{ id: "", data: { content: <p>Test</p>, header: "Test" } }]);

  const notificationContext = useContext(NotificationContext);
  const { alerts } = notificationContext!;
  const [showWidgetCatalog, setShowWidgetCatalog] = useState(false);

  function openWidgetCatalog() {
    setShowWidgetCatalog(true);
  }

  return (
    <AppLayout
      contentType="dashboard"
      navigation={<Sidenav active="/" />}
      notifications={alerts()}
      toolsHide={true}
      splitPanelPreferences={{ position: "side" }}
      splitPanelOpen={showWidgetCatalog}
      onSplitPanelToggle={() => setShowWidgetCatalog(!showWidgetCatalog)}
      splitPanel={
        <SplitPanel
          closeBehavior="hide"
          hidePreferencesButton={true}
          header="Add Widget"
        >
          <Button>Test</Button>
        </SplitPanel>
      }
      content={
        <ContentLayout
          header={
            <Header
              variant="h1"
              actions={
                <SpaceBetween size="s">
                  <Button onClick={openWidgetCatalog} iconName="add-plus">
                    Add Widget
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
                settings={
                  <ButtonDropdown
                    variant="icon"
                    items={[{ id: "remove", text: "Remove" }]}
                  ></ButtonDropdown>
                }
                header={<Header>{item.data.header}</Header>}
                i18nStrings={{
                  dragHandleAriaLabel: "Drag Handle",
                  resizeHandleAriaLabel: "Resize Handle",
                }}
              >
                {item.data.content}
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
              console.log(event.detail.items);
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
