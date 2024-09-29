import axios from "axios";
import { Repo } from "../models/repo.ts";
import { useEffect, useState } from "react";
import {
  Box,
  Button,
  FormField,
  Header,
  Input,
  Modal,
  Select,
  SelectProps,
  SpaceBetween,
  Table,
  Toggle,
} from "@cloudscape-design/components";

function ReposPage() {
  const [repos, setRepos] = useState<Repo[]>([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [repoName, setRepoName] = useState("");
  const [repoType, setRepoType] = useState<SelectProps.Option>({
    value: "Cargo",
  });
  const [repoPublic, setRepoPublic] = useState(false);

  const selectOptions = [{ value: "Cargo" }];
  const [selectedRepo, setSelectedRepo] = useState<Repo[]>([]);

  useEffect(() => {
    load_repos();
  }, []);

  function load_repos() {
    setLoading(true);
    axios.get("http://localhost:6300/repo").then((response) => {
      setRepos(response.data);
      setLoading(false);
    });
  }

  function create_repo() {
    axios
      .post("http://localhost:6300/repo", {
        name: repoName,
        repo_type: repoType.value,
        public: repoPublic,
      })
      .then(() => {
        setRepoType({ value: "Cargo" });
        setRepoName("");
        setRepoPublic(false);
        load_repos();
        setShowModal(false);
      });
  }

  function delete_repos() {
    for (const repo of selectedRepo) {
      axios.delete(`http://localhost:6300/repo/${repo.id}`).then(() => {
        setSelectedRepo([]);
        load_repos();
      });
    }
  }

  function show_create_repo_dialog() {
    setShowModal(true);
  }

  return (
    <>
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
            <SpaceBetween alignItems="center" size="xs" direction="horizontal">
              <Input
                value={repoName}
                onChange={({ detail }) => setRepoName(detail.value)}
              />
            </SpaceBetween>
          </FormField>

          <FormField label="Type">
            <SpaceBetween alignItems="center" size="xs" direction="horizontal">
              <Select
                options={selectOptions}
                selectedOption={repoType}
                onChange={({ detail }) => setRepoType(detail.selectedOption)}
              />
            </SpaceBetween>
          </FormField>
          <FormField label="Public">
            <SpaceBetween alignItems="center" size="xs" direction="horizontal">
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
        selectionType={"multi"}
        selectedItems={selectedRepo}
        onSelectionChange={({ detail }) => {
          setSelectedRepo(detail.selectedItems);
        }}
        columnDefinitions={[
          {
            id: "id",
            header: "ID",
            cell: (repo) => repo.id,
            sortingField: "id",
          },
          { id: "name", header: "Name", cell: (repo) => repo.name },
          { id: "repo_type", header: "Type", cell: (repo) => repo.repo_type },
          {
            id: "public",
            header: "Public",
            cell: (repo) => String(repo.public),
          },
        ]}
        header={
          <SpaceBetween size="m">
            <Header
              actions={
                <SpaceBetween direction="horizontal" size="m">
                  <Button
                    disabled={selectedRepo.length == 0}
                    onClick={delete_repos}
                  >
                    Delete Repositories
                  </Button>
                  <Button variant={"primary"} onClick={show_create_repo_dialog}>
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
    </>
  );
}

export default ReposPage;
