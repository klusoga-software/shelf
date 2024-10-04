import { useEffect, useState } from "react";
import {
  Box,
  Button,
  Checkbox,
  CopyToClipboard,
  DatePicker,
  FormField,
  Header,
  Input,
  Link,
  Modal,
  Multiselect,
  Select,
  SelectProps,
  SpaceBetween,
  Table,
} from "@cloudscape-design/components";
import { ServiceAccount } from "../models/service-account.ts";
import axios from "axios";
import { Role } from "../models/role.ts";
import { Repo } from "../models/repo.ts";

function ServiceAccountsPage() {
  const [loading, setLoading] = useState(true);
  const [serviceAccounts, setServiceAccounts] = useState<ServiceAccount[]>([]);
  const [selectedServiceAccounts, setSelectedServiceAccounts] = useState<
    ServiceAccount[]
  >([]);
  const [showModal, setShowModal] = useState(false);
  const [expiration, setExpiration] = useState("");
  const [noExpiration, setNoExpiration] = useState(false);
  const [name, setName] = useState("");
  const [repos, setRepos] = useState<SelectProps.Option[]>([]);
  const [selectedRepos, setSelectedRepos] = useState<
    readonly SelectProps.Option[]
  >([]);
  const [roles, setRoles] = useState<SelectProps.Option[]>([]);
  const [selectedRole, setSelectedRole] =
    useState<Map<string, SelectProps.Option>>();

  const [showSecretModal, setShowSecretModal] = useState(false);
  const [secret, setSecret] = useState("");

  useEffect(() => {
    load_service_accounts();
    load_roles();
    load_repos();
  }, []);

  function load_service_accounts() {
    setLoading(true);
    axios.get("/api/service-accounts").then((response) => {
      setServiceAccounts(response.data);
      setLoading(false);
    });
  }
  function delete_service_account() {
    for (const sa of selectedServiceAccounts) {
      axios.delete(`/api/service-accounts/${sa.id}`).then(() => {
        load_service_accounts();
      });
    }
  }
  function create_service_account() {
    const mapping: [number, number][] = [];

    selectedRole!.forEach((value, key) => {
      mapping.push([parseInt(key), parseInt(value.value as string)]);
    });

    axios
      .post("/api/service-accounts", {
        name: name,
        repo_list: mapping,
        expired_at: noExpiration ? null : expiration,
      })
      .then((response) => {
        setSecret(response.data.secret);
        setShowSecretModal(true);
        setExpiration("");
        setSelectedRole(new Map());
        setSelectedRepos([]);
        setShowModal(false);
        load_service_accounts();
      });
  }

  function load_roles() {
    axios.get<Role[]>("/api/roles").then((response) => {
      const newRoles = response.data.map((role) => ({
        label: role.name,
        value: role.id.toString(),
      }));
      setRoles(newRoles);
      setLoading(false); // We set loading false after the data is loaded
    });
  }

  function load_repos() {
    axios.get<Repo[]>("/api/repos").then((response) => {
      const newRepos = response.data.map((repo) => ({
        label: repo.name,
        value: repo.id.toString(),
      }));
      setRepos(newRepos);
      setLoading(false); // We set loading false after the data is loaded
    });
  }

  function repo_roles(): JSX.Element[] {
    const repos_fields: JSX.Element[] = [];
    for (const repo of selectedRepos) {
      repos_fields.push(
        <FormField key={repo.value} label={`${repo.label} role`}>
          <Select
            onChange={({ detail }) =>
              setSelectedRole((prevState) => {
                const newMap = new Map(prevState);
                newMap.set(repo.value!, detail.selectedOption);
                return newMap;
              })
            }
            options={roles}
            // @ts-expect-error TODO: will be fixed later
            selectedOption={selectedRole?.get(repo.value!)}
          ></Select>
        </FormField>,
      );
    }
    return repos_fields;
  }

  return (
    <>
      <Modal
        visible={showSecretModal}
        onDismiss={() => setShowSecretModal(false)}
        header="Service Account Credentials"
        footer={
          <Box>
            <SpaceBetween size="xs" direction="horizontal">
              <Button onClick={() => setShowSecretModal(false)}>Ok</Button>
            </SpaceBetween>
          </Box>
        }
      >
        <SpaceBetween size="m">
          <FormField label="Secret">
            <CopyToClipboard
              variant="inline"
              textToCopy={secret}
              copySuccessText="Secret copied"
              copyErrorText="Error while copy secret"
            ></CopyToClipboard>
          </FormField>
        </SpaceBetween>
      </Modal>
      <Modal
        header="Create Service Account"
        onDismiss={() => setShowModal(false)}
        visible={showModal}
        footer={
          <Box float="right">
            <SpaceBetween direction="horizontal" size="xs">
              <Button
                variant="primary"
                onClick={() => create_service_account()}
              >
                Create
              </Button>
            </SpaceBetween>
          </Box>
        }
      >
        <SpaceBetween size="m">
          <FormField label="Name">
            <Input
              onChange={({ detail }) => setName(detail.value)}
              value={name}
            ></Input>
          </FormField>
          <SpaceBetween alignItems="center" size="xxl" direction="horizontal">
            <FormField label="Expiration">
              <DatePicker
                disabled={noExpiration}
                onChange={({ detail }) => {
                  const date = new Date(detail.value);
                  setExpiration(date.toISOString());
                }}
                value={expiration}
              ></DatePicker>
            </FormField>
            <FormField label="No Expiration">
              <Checkbox
                onChange={({ detail }) => {
                  setNoExpiration(detail.checked);
                }}
                checked={noExpiration}
              ></Checkbox>
            </FormField>
          </SpaceBetween>
          <FormField label="Repos">
            <Multiselect
              onChange={({ detail }) =>
                setSelectedRepos(detail.selectedOptions)
              }
              options={repos}
              selectedOptions={selectedRepos}
            ></Multiselect>
          </FormField>
          {repo_roles()}
        </SpaceBetween>
      </Modal>
      <Table
        header={
          <SpaceBetween size="m">
            <Header
              actions={
                <SpaceBetween direction="horizontal" size="m">
                  <Button
                    disabled={selectedServiceAccounts.length == 0}
                    onClick={delete_service_account}
                  >
                    Delete Service Accounts
                  </Button>
                  <Button variant="primary" onClick={() => setShowModal(true)}>
                    Create Service Account
                  </Button>
                </SpaceBetween>
              }
            >
              Service Accounts
            </Header>
          </SpaceBetween>
        }
        loading={loading}
        selectedItems={selectedServiceAccounts}
        items={serviceAccounts}
        selectionType={"multi"}
        onSelectionChange={({ detail }) =>
          setSelectedServiceAccounts(detail.selectedItems)
        }
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
