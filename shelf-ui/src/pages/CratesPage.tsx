import { useContext, useEffect, useState } from "react";
import Crate from "../models/crate.ts";
import axios from "axios";
import { useParams } from "react-router-dom";
import {
  Button,
  Header,
  SpaceBetween,
  Table,
} from "@cloudscape-design/components";
import { useAuth } from "react-oidc-context";
import { NotificationContext } from "../components/NotificationProvider.tsx";

function CratesPage() {
  const [crates, setCrates] = useState<Crate[]>([]);
  const [loading, setLoading] = useState(true);
  const params = useParams();
  const [selectedCrates, setSelectedCrates] = useState<Crate[]>([]);

  const auth = useAuth();

  const notificationContext = useContext(NotificationContext);
  const { showNotification } = notificationContext!;

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
    for (const crate of selectedCrates) {
      axios
        .delete(`/api/crate/${crate.id}`, {
          headers: { Authorization: `Bearer ${auth.user?.access_token}` },
        })
        .then(() => {
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
    <>
      <Table
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
          { id: "version", header: "Version", cell: (crate) => crate.version },
          { id: "size", header: "Size", cell: (crate) => crate.crate_size },
        ]}
        header={
          <SpaceBetween size="m">
            <Header
              actions={
                <SpaceBetween direction="horizontal" size="m">
                  <Button
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
    </>
  );
}

export default CratesPage;
