import axios from "axios";
import { Repo } from "../models/repo.ts";
import { useContext, useEffect, useState } from "react";
import {
  AppLayout,
  Box,
  Button,
  ContentLayout,
  CopyToClipboard,
  FormField,
  Header,
  Input,
  Link,
  Modal,
  Select,
  SelectProps,
  SpaceBetween,
  Table,
  Toggle,
} from "@cloudscape-design/components";
import { useNavigate } from "react-router-dom";
import { useAuth } from "react-oidc-context";
import { NotificationContext } from "../components/NotificationProvider.tsx";
import Sidenav from "../components/Sidenav.tsx";

function ReposPage() {
  const [repos, setRepos] = useState<Repo[]>([]);
  const [loading, setLoading] = useState(true);
  const [buttonLoading, setButtonLoading] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [repoName, setRepoName] = useState("");
  const [repoType, setRepoType] = useState<SelectProps.Option>({
    value: "Cargo",
  });
  const [repoPublic, setRepoPublic] = useState(false);

  const selectOptions = [{ value: "Cargo" }];
  const [selectedRepo, setSelectedRepo] = useState<Repo[]>([]);
  const navigate = useNavigate();

  const auth = useAuth();
  const notificationContext = useContext(NotificationContext);
  const { showNotification, alerts } = notificationContext!;

  useEffect(() => {
    if (auth.user) {
      load_repos();
    }
  }, [auth]);

  function load_repos() {
    setLoading(true);
    axios
      .get("/api/repos", {
        headers: { Authorization: `Bearer ${auth.user?.access_token}` },
      })
      .then((response) => {
        setRepos(response.data);
        setLoading(false);
      })
      .catch((err) => {
        showNotification({
          type: "error",
          header: "Error while load repos",
          message: err.response?.data,
        });
      });
  }

  function create_repo() {
    axios
      .post(
        "/api/repos",
        {
          name: repoName,
          repo_type: repoType.value,
          public: repoPublic,
        },
        { headers: { Authorization: `Bearer ${auth.user?.access_token}` } },
      )
      .then(() => {
        setRepoType({ value: "Cargo" });
        showNotification({
          type: "success",
          header: "Repository created!",
          message: `Successfully created ${repoName}`,
        });
        setRepoName("");
        setRepoPublic(false);
        load_repos();
        setShowModal(false);
      })
      .catch((err) => {
        showNotification({
          type: "error",
          header: "Error while delete Repo",
          message: err.response?.data,
        });
      });
  }

  function delete_repos() {
    setButtonLoading(true);
    for (const repo of selectedRepo) {
      axios
        .delete(`/api/repos/${repo.id}`, {
          headers: { Authorization: `Bearer ${auth.user?.access_token}` },
        })
        .then(() => {
          setButtonLoading(false);
          setSelectedRepo([]);
          load_repos();
        })
        .catch((err) => {
          showNotification({
            type: "error",
            header: "Error while delete repo",
            message: err.response?.data,
          });
        });
    }
  }

  function show_create_repo_dialog() {
    setShowModal(true);
  }

  function receive_config(repo: Repo): string {
    return `${repo.name} = { index = "sparse+${axios.defaults.baseURL ?? `${window.location.protocol}//${window.location.host}/`}cargo/${repo.name}/index/" }`;
  }

  return (
    <AppLayout
      contentType="table"
      navigation={<Sidenav active="/repos" />}
      notifications={alerts()}
      toolsHide={true}
      content={
        <ContentLayout>
          <Modal
            visible={showModal}
            onDismiss={() => setShowModal(false)}
            header="Create Repository"
            footer={
              <Box float="right">
                <SpaceBetween direction="horizontal" size="xs">
                  <Button variant="primary" onClick={() => create_repo()}>
                    Create
                  </Button>
                </SpaceBetween>
              </Box>
            }
          >
            <SpaceBetween size="m">
              <FormField label="Name">
                <SpaceBetween
                  alignItems="center"
                  size="xs"
                  direction="horizontal"
                >
                  <Input
                    value={repoName}
                    onChange={({ detail }) => setRepoName(detail.value)}
                  />
                </SpaceBetween>
              </FormField>

              <FormField label="Type">
                <SpaceBetween
                  alignItems="center"
                  size="xs"
                  direction="horizontal"
                >
                  <Select
                    options={selectOptions}
                    selectedOption={repoType}
                    onChange={({ detail }) =>
                      setRepoType(detail.selectedOption)
                    }
                  />
                </SpaceBetween>
              </FormField>
              <FormField label="Public">
                <SpaceBetween
                  alignItems="center"
                  size="xs"
                  direction="horizontal"
                >
                  <Toggle
                    checked={repoPublic}
                    onChange={() => setRepoPublic(!repoPublic)}
                  />
                </SpaceBetween>
              </FormField>
            </SpaceBetween>
          </Modal>
          <Table
            items={repos}
            variant="full-page"
            selectionType={"multi"}
            selectedItems={selectedRepo}
            onSelectionChange={({ detail }) => {
              setSelectedRepo(detail.selectedItems);
            }}
            columnDefinitions={[
              {
                id: "id",
                header: "ID",
                cell: (repo) => (
                  <Link
                    onFollow={(e) => {
                      e.preventDefault();
                      switch (repo.repo_type) {
                        case "Cargo":
                          navigate(`/crates/${repo.id}`);
                      }
                    }}
                  >
                    {repo.id}
                  </Link>
                ),
                sortingField: "id",
              },
              { id: "name", header: "Name", cell: (repo) => repo.name },
              {
                id: "repo_type",
                header: "Type",
                cell: (repo) => repo.repo_type,
              },
              {
                id: "public",
                header: "Public",
                cell: (repo) => String(repo.public),
              },
              {
                id: "action",
                header: "config",
                cell: (repo) => (
                  <CopyToClipboard
                    variant="inline"
                    textToCopy={receive_config(repo)}
                    copySuccessText="Config copied"
                    copyErrorText="Could not copy config"
                  />
                ),
              },
            ]}
            header={
              <SpaceBetween size="m">
                <Header
                  actions={
                    <SpaceBetween direction="horizontal" size="m">
                      <Button
                        loading={buttonLoading}
                        disabled={selectedRepo.length == 0}
                        onClick={delete_repos}
                      >
                        Delete Repositories
                      </Button>
                      <Button
                        variant={"primary"}
                        onClick={show_create_repo_dialog}
                      >
                        Create Repository
                      </Button>
                    </SpaceBetween>
                  }
                >
                  Repositories
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

export default ReposPage;
