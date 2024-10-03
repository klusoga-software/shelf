import { useEffect, useState } from "react";
import { Link, Table } from "@cloudscape-design/components";
import { ServiceAccount } from "../models/service-account.ts";
import axios from "axios";

function ServiceAccountsPage() {
  const [loading, setLoading] = useState(true);
  const [serviceAccounts, setServiceAccounts] = useState<ServiceAccount[]>([]);

  useEffect(() => {
    setLoading(true);
    load_service_accounts();
  }, []);

  function load_service_accounts() {
    axios.get("/api/service-accounts").then((response) => {
      setServiceAccounts(response.data);
      setLoading(false);
    });
  }

  return (
    <>
      <Table
        loading={loading}
        items={serviceAccounts}
        columnDefinitions={[
          {
            id: "id",
            header: "ID",
            cell: (sa) => (
              <Link
                onFollow={(e) => {
                  e.preventDefault();
                }}
              >
                {sa.id}
              </Link>
            ),
            sortingField: "id",
          },
          { id: "name", header: "Name", cell: (sa) => sa.name },
          {
            id: "created_at",
            header: "Created At",
            cell: (sa) => sa.created_at.toString(),
          },
          {
            id: "updated_at",
            header: "Updated At",
            cell: (sa) => sa.updated_at.toString(),
          },
          {
            id: "expires_at",
            header: "Expires At",
            cell: (sa) =>
              sa.expires_at ? sa.expires_at.toString() : "No expiration",
          },
          { id: "repo_count", header: "Repos", cell: (sa) => sa.repo_count },
        ]}
      ></Table>
    </>
  );
}

export default ServiceAccountsPage;
