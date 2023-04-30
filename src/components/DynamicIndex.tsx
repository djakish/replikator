import { invoke } from "@tauri-apps/api/tauri";

// my imports
import {
  useCurrentState,
  Card,
  Textarea,
  Input,
  Progress,
  Page,
  Spacer,
  Text,
  Button,
  Grid,
} from "@geist-ui/core";
import { open } from "@tauri-apps/api/dialog";
import { useState } from "react";
import RouterButtons from "@/components/RouterButtons";

import { appWindow } from "@tauri-apps/api/window";
import Controls from "./Controls";

import LogOutput from "./LogOutput";

interface Payload {
  message: string;
}
type ProgressHandler = (progress: number, total: number) => void;

export default function DynamicIndex() {
  const [path, setPath] = useState<string | undefined>("");
  const [fileTreeText, setFileTreeText, fileTreeTextRef] = useCurrentState<
    string | undefined
  >("Logs will be here");
  const [maxFileTreeCount, setMaxFileTreeCount] = useState<number>(0);
  const [fileTreeCount, setFileTreeCount, fileTreeCountRef] =
    useCurrentState<number>(0);
  const [fileTreePercentage, setFileTreePercentage] = useState<number>(0);

  const [isLoading, setIsLoading] = useState(false);
  const [fileTreeLoading, setFileTreeLoading, fileTreeLoadingRef] =
    useCurrentState(false);

  async function openDir() {
    setFileTreeLoading(true);
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: "~/",
    });
    if ((await selected) === null) {
      // user cancelled the selection
    } else {
      // @ts-ignore
      await setPath(await selected);
      let file_tree_result: [string, number] = await invoke("get_files_tree", {
        path: await selected,
      });
      await setFileTreeText(file_tree_result[0]);
      await setMaxFileTreeCount(file_tree_result[1]);
      await setFileTreeCount(0);
      await setFileTreePercentage(0);
    }
    setFileTreeLoading(false);
  }

  async function backup() {
    setIsLoading(true);
    const selected = open({
      directory: true,
      multiple: false,
      defaultPath: "~/",
    });
    if (selected === null) {
      // user cancelled the selection
    } else {
      await setFileTreeText("BACKUP STARTED \n");
      await listenToEventIfNeeded("compress://progress");
      let result: string = await invoke("compress_files", {
        input: path,
        output: await selected,
      });
      setFileTreeText(fileTreeTextRef.current + result + "\n");
    }
    setIsLoading(false);
  }

  const handlers: Map<number, ProgressHandler> = new Map();
  let listening = false;

  async function listenToEventIfNeeded(event: string): Promise<void> {
    if (listening) {
      return await Promise.resolve();
    }
    return await appWindow
      .listen<Payload>(event, ({ payload }) => {
        setFileTreeCount(fileTreeCountRef.current + 1);
        setFileTreeText(fileTreeTextRef.current + payload.message + "\n");

        invoke("get_percentage_rounded", {
          x: fileTreeCountRef.current,
          y: maxFileTreeCount,
        })
          .then((res) => {
            let num = res as number;
            if (num != fileTreePercentage) setFileTreePercentage(res as number);
          })
          .catch((e) => console.error(e));
      })
      .then(() => {
        listening = true;
      });
  }

  return (
    <Page render="effect-seo">
      <Controls />
      <Grid.Container gap={2} justify="flex-start">
        <RouterButtons />
        <Grid sm={22}>
          <Card hoverable shadow width="100%">
            <Text h4 my={0}>
              Create a backup
            </Text>
            <Spacer h={0.5} />
            <Grid.Container gap={2}>
              <Grid sm={18}>
                <Input readOnly value={path} width="100%" />
              </Grid>
              <Grid>
                <Button
                  loading={fileTreeLoading}
                  onClick={openDir}
                  auto
                  scale={0.75}
                  width="100%"
                >
                  {" "}
                  Pick a directory{" "}
                </Button>
              </Grid>
            </Grid.Container>
            <Spacer />
            <LogOutput text={fileTreeTextRef.current}></LogOutput>
            <Spacer />
            <Button
              loading={isLoading}
              type="success-light"
              onClick={backup}
              scale={0.75}
            >
              backup
            </Button>
            <Card.Footer>
              <Progress value={fileTreePercentage} max={100} />
            </Card.Footer>
          </Card>
        </Grid>
      </Grid.Container>
    </Page>
  );
}
