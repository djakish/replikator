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
  useToasts,
  ToastLayout,
} from "@geist-ui/core";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import RouterButtons from "@/components/RouterButtons";
import Controls from "./Controls";
import React from "react";

import { BackupEntry} from "@/lib/types";
import { json_path } from "@/lib/utils";
export default function DynamicDatabase() {
  const { setVisible, bindings } = useModal();

  const layout: ToastLayout = {
    maxHeight: '120px',
    maxWidth: '120vw',
    width: '60vw',
    padding: '20px'
  }
  const { setToast } = useToasts(layout);

  const [data, setData] = React.useState<BackupEntry[]>([]);

  const [inputPath, setInputPath] = useState<string>("");
  const [outputPath, setOutputPath] = useState<string>("");
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

    if (backupTitle.length == 0) {
      setVisible(false);

      setToast({
        text: 'Title can\'t be empty',
        type: 'warning',
      })
      return;
    }
    if (inputPath.length == 0) {
      setVisible(false);

      setToast({
        text: 'Input can\'t be empty',
        type: 'warning',
      })
      return;
    }
    if (outputPath.length == 0) {
      setVisible(false);

      setToast({
        text: 'Output can\'t be empty',
        type: 'warning',
      })
      return;
    }
    
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
      invoke("notify_start")
      invoke("increment", {
        jsonPath: await json_path(),
        hash: rowData.hash,
        input: rowData.input,
        output: rowData.output,
        lastBackup: rowData.lastBackup,
      });

      await fetchData().catch(console.error);
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

  const dateRender = (value: any, rowData: any, index: number) => {
    return (
      <Text p b>
        {
          // @ts-ignore
          new Date(rowData.lastBackup).toLocaleString("en-GB")
        }
      </Text>
    );
  };

  const ShowPaths = (value: any, rowData: any, index: number) => {
    let text  = "input: " + rowData.input + " | output: " + rowData.output
    const click = () =>
      setToast({
        text: text, 
        delay: 4000
      });
    return (
      <Button
        auto
        scale={1 / 3}
        font="12px"
        onClick={click}
      >Show paths</Button>
    );
  };

  return (
    <Page render="effect">
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
              <Table.Column
                prop="lastBackup"
                label="last backup"
                render={dateRender}
              />
              <Table.Column prop="input" render={ShowPaths}  width={25} />
              <Table.Column prop="backup" render={backupAction} width={25}  />
              <Table.Column prop="delete" render={deleteAction} width={25} />
            </Table>
            <Spacer />
          </Card>
        </Grid>
      </Grid.Container>
    </Page>
  );
}
