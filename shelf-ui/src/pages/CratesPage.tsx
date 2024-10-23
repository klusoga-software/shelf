import { useContext, useEffect, useState } from "react";
import Crate from "../models/crate.ts";
import axios from "axios";
import { useParams } from "react-router-dom";
import {
  AppLayout,
  Box,
  Button,
  ContentLayout,
  Header,
  SpaceBetween,
  Table,
} from "@cloudscape-design/components";
import { useAuth } from "react-oidc-context";
import { NotificationContext } from "../components/NotificationProvider.tsx";
import Sidenav from "../components/Sidenav.tsx";

function CratesPage() {
  const [crates, setCrates] = useState<Crate[]>([]);
  const [loading, setLoading] = useState(true);
  const [buttonLoading, setButtonLoading] = useState(false);
  const params = useParams();
  const [selectedCrates, setSelectedCrates] = useState<Crate[]>([]);

  const auth = useAuth();

  const notificationContext = useContext(NotificationContext);
  const { showNotification, alerts } = notificationContext!;

  useEffect(() => {
    if (auth.user) {
      get_crates();
    }
  }, [auth]);

  function get_crates() {
    setLoading(true);
    axios
      .get(`/api/crate/${params.id}`, {
        headers: { Authorization: `Bearer ${auth.user?.access_token}` },
      })
      .then((res) => {
        setCrates(res.data);
        setLoading(false);
      })
      .catch((err) => {
        showNotification({
          type: "error",
          header: "Error while get crates",
          message: err.response?.data,
        });
      });
  }

  function delete_crate() {
    setButtonLoading(true);
    for (const crate of selectedCrates) {
      axios
        .delete(`/api/crate/${crate.id}`, {
          headers: { Authorization: `Bearer ${auth.user?.access_token}` },
        })
        .then(() => {
          setButtonLoading(false);
          get_crates();
          setSelectedCrates([]);
        })
        .catch((err) => {
          showNotification({
            type: "error",
            header: "Error while delete crate",
            message: err.response?.data,
          });
        });
    }
  }

  return (
    <AppLayout
      contentType="table"
      navigation={<Sidenav active="/repos" />}
      notifications={alerts()}
      toolsHide={true}
      content={
        <ContentLayout>
          <Table
            empty={<Box>No crates found</Box>}
            variant="full-page"
            items={crates}
            selectionType={"multi"}
            selectedItems={selectedCrates}
            onSelectionChange={({ detail }) => {
              setSelectedCrates(detail.selectedItems);
            }}
            columnDefinitions={[
              {
                id: "id",
                header: "ID",
                cell: (crate) => crate.id,
                sortingField: "id",
              },
              { id: "name", header: "Name", cell: (crate) => crate.name },
              {
                id: "version",
                header: "Version",
                cell: (crate) => crate.version,
              },
              { id: "size", header: "Size", cell: (crate) => crate.crate_size },
            ]}
            header={
              <SpaceBetween size="m">
                <Header
                  actions={
                    <SpaceBetween direction="horizontal" size="m">
                      <Button
                        loading={buttonLoading}
                        disabled={selectedCrates.length == 0}
                        onClick={delete_crate}
                      >
                        Delete Crates
                      </Button>
                    </SpaceBetween>
                  }
                >
                  Crates
                </Header>
              </SpaceBetween>
            }
            loading={loading}
          ></Table>
        </ContentLayout>
      }
    ></AppLayout>
  );
}

export default CratesPage;
