import { useEffect, useState } from "react";
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

function CratesPage() {
  const [crates, setCrates] = useState<Crate[]>([]);
  const [loading, setLoading] = useState(true);
  const params = useParams();
  const [selectedCrates, setSelectedCrates] = useState<Crate[]>([]);

  const auth = useAuth();

  useEffect(() => {
    get_crates();
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
