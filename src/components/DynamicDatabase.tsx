import { invoke } from "@tauri-apps/api/tauri";

// my imports
import {
  Modal,
  Card,
  Input,
  Page,
  Spacer,
  Text,
  Button,
  Grid,
  Table,
  useModal,
  Radio,
} from "@geist-ui/core";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import RouterButtons from "@/components/RouterButtons";
import { appWindow } from "@tauri-apps/api/window";
import Controls from "./Controls";
import React from "react";

import { BackupEntry, BackupReturn, UpdateTime } from "@/lib/types";
import { json_path } from "@/lib/utils";

export default function DynamicDatabase() {
  const { setVisible, bindings } = useModal();

  const [data, setData] = React.useState<BackupEntry[]>([]);

  const [inputPath, setInputPath] = useState<string | undefined>("");
  const [outputPath, setOutputPath] = useState<string | undefined>("");
  const [backupTitle, setBackupTitle] = useState<string>("");
  const [updateTime, setUpdateTime] = useState(9999);

  async function openInputDir() {
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: "~/",
    });
    if ((await selected) === null) {
      // user cancelled the selection
      await setInputPath("");
    } else {
      // @ts-ignore
      await setInputPath(await selected);
    }
  }

  async function openOutputDir() {
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: "~/",
    });
    if ((await selected) === null) {
      // user cancelled the selection
      await setOutputPath("");
    } else {
      // @ts-ignore
      await setOutputPath(await selected);
    }
  }

  async function addEntry() {
    await invoke("add_entry", {
      jsonPath: await json_path(),
      title: backupTitle,
      input: inputPath,
      output: outputPath,
      nextUpdate: updateTime,
    });

    setVisible(false);
    fetchData().catch(console.error);
  }

  const fetchData = async () => {
    const data: string = await invoke("get_table", {
      jsonPath: await json_path(),
    });
    var jsonData = JSON.parse(data);
    setData(jsonData.backups);
  };

  useEffect(() => {
    fetchData().catch(console.error);
  }, []);

  const backupAction = (value: any, rowData: any, index: number) => {
    const backupHandler = async () => {
      //setData((last) => last.filter((_, dataIndex) => dataIndex !== index));
      console.log("test")
      invoke("increment", {
        jsonPath: await json_path(),
        hash: rowData.hash,
        input: rowData.input,
        output: rowData.output,
        lastBackup: rowData.lastBackup
      });
      fetchData().catch(console.error);
    };
    return (
      <Button
        type="success-light"
        auto
        scale={1 / 3}
        font="12px"
        onClick={backupHandler}
      >
        Backup
      </Button>
    );
  };

  const deleteAction = (value: any, rowData: any, index: number) => {
    const removeHandler = async () => {
      invoke("delete_entry", {
        jsonPath: await json_path(),
        hash: rowData.hash,
      });
      fetchData().catch(console.error);
    };
    return (
      <Button
        type="error"
        auto
        scale={1 / 3}
        font="12px"
        onClick={removeHandler}
      >
        Remove
      </Button>
    );
  };

  return (
    <Page render="effect-seo">
      <Controls />
      <Grid.Container gap={2} justify="flex-start">
        <RouterButtons />
        <Grid sm={22}>
          <Card hoverable shadow width="100%">
            <Text h4 my={0}>
              My Backups
            </Text>
            <Spacer />
            <Button
              auto
              onClick={() => setVisible(true)}
              type="success-light"
              scale={0.75}
            >
              create a new backup
            </Button>
            <Modal width="35rem" {...bindings}>
              <Modal.Title>Add a new backup</Modal.Title>
              <Modal.Content>
                <Input
                  readOnly
                  value={inputPath}
                  label="input"
                  placeholder="C:\"
                  width="100%"
                  onClick={openInputDir}
                />
                <Spacer />
                <Input
                  readOnly
                  value={outputPath}
                  label="output"
                  placeholder="G:\"
                  width="100%"
                  onClick={openOutputDir}
                />
                <Spacer />
                <Input
                  label="backup name"
                  placeholder="My photos"
                  width="100%"
                  value={backupTitle}
                  onChange={(val) => setBackupTitle(val.target.value)}
                />
                <Spacer />
                <Text p b>
                  Select an interval at which backups will run (important to
                  have the app open in the background).
                </Text>
                <Radio.Group
                  value="9999"
                  useRow
                  onChange={(val) => setUpdateTime(val as number)}
                >
                  <Radio value="9999">Never</Radio>
                  <Radio value="7">Every week</Radio>
                  <Radio value="30">Every Month</Radio>
                </Radio.Group>
                <Spacer />
                <Spacer />
                <Grid.Container justify="center">
                  <Button auto type="success-light" onClick={addEntry}>
                    Add
                  </Button>
                </Grid.Container>
              </Modal.Content>
            </Modal>

            <Spacer />
            <Table data={data}>
              <Table.Column prop="title" label="backup name" />
              <Table.Column prop="lastBackup" label="last backup" />
              <Table.Column prop="backup" width={50} render={backupAction} />
              <Table.Column
                prop="delete"
                label=""
                width={50}
                render={deleteAction}
              />
            </Table>
            <Spacer />
          </Card>
        </Grid>
      </Grid.Container>
    </Page>
  );
}
