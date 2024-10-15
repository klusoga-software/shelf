import {
  Box,
  Button,
  Header,
  SpaceBetween,
} from "@cloudscape-design/components";
import {
  Board,
  BoardItem,
  BoardProps,
} from "@cloudscape-design/board-components";
import { useState } from "react";
import { DashboardItemData } from "../models/dashboard-item-data.ts";

function Dashboard() {
  const [items, setItems] = useState<
    readonly BoardProps.Item<DashboardItemData>[]
  >([{ id: "", data: { content: <p>Test</p>, header: "Test" } }]);

  return (
    <>
      <SpaceBetween size="m">
        <Header variant="h1">Dashboard</Header>
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
            setItems(event.detail.items);
          }}
          empty={
            <Box margin={{ vertical: "xs" }} textAlign="center" color="inherit">
              <SpaceBetween size="m">
                <Box variant="strong" color="inherit">
                  No items
                </Box>
                <Button iconName="add-plus">Add an item</Button>
              </SpaceBetween>
            </Box>
          }
        ></Board>
      </SpaceBetween>
    </>
  );
}

export default Dashboard;
