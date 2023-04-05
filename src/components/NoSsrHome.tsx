import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect } from 'react'

// my imports
import { useCurrentState, Card, Tooltip, Loading, Textarea, ButtonGroup, Tree, Input, Progress, Page, Spacer, Text, Button, Grid } from '@geist-ui/core';
import { Github, Save, Rewind, Database } from '@geist-ui/icons';
import { open } from '@tauri-apps/api/dialog';
import { useState } from 'react';
import Link from 'next/link'
import RouterButtons from '@/components/RouterButtons'

import { appWindow } from "@tauri-apps/api/window";

const inter = Inter({ subsets: ['latin'] })


interface Payload {
  message: string;
}
type ProgressHandler = (progress: number, total: number) => void;



export default function NoSsrHome() {
  const [path, setPath] = useState<string | undefined>('');
  const [fileTreeText, setFileTreeText, fileTreeTextRef] = useCurrentState<string | undefined>('');
  const [maxFileTreeCount, setMaxFileTreeCount] = useState<number>(0);
  const [fileTreeCount, setFileTreeCount, fileTreeCountRef] = useCurrentState<number>(0);


  const [isLoading, setIsLoading] = useState(false);
  const [fileTreeLoading, setFileTreeLoading, fileTreeLoadingRef] = useCurrentState(false)

  async function openDir() {
    setFileTreeLoading(true)
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: '~/',
    }); if (await selected === null) {
      // user cancelled the selection
    } else {
      // @ts-ignore       
      await setPath(await selected)
      let file_tree_result: [string, number] = await invoke('get_files_tree', { path: await selected })
      await setFileTreeText(file_tree_result[0])
      await setMaxFileTreeCount(file_tree_result[1] )
      await setFileTreeCount(0)
    }
    setFileTreeLoading(false)
  }


  async function backup() {
    setFileTreeLoading(false)
    const selected = open({
      directory: true,
      multiple: false,
      defaultPath: '~/',
    }); if (selected === null) {
      // user cancelled the selection
    } else {
      await setFileTreeText('')
      await listenToEventIfNeeded("compress://progress")
      await invoke('compress_files', { input: path, output: await selected })
    }
  }

  const handlers: Map<number, ProgressHandler> = new Map();
  let listening = false;

  async function listenToEventIfNeeded(event: string): Promise<void> {
    if (listening) {
      return await Promise.resolve();
    }
    return await appWindow
      .listen<Payload>(event, ({ payload }) => {
        setFileTreeCount(fileTreeCountRef.current+1);
        setFileTreeText(fileTreeTextRef.current + payload.message + "\n");
      })
      .then(() => {
        listening = true;
      });
  }


  return (
    <Page >
      <Grid.Container height="100%">
        <RouterButtons />
        <Grid sm={22} height="100%" >
          <Card shadow width="100%" >
            <Text h4 my={0}>Create a backup</Text>
            <Spacer h={.5} />
            <Grid.Container gap={2}>
              <Grid md={24} sm={14} xs={22}>
                <Input readOnly value={path} width="100%" />
              </Grid>
              <Grid>
                <Button loading={fileTreeLoading} onClick={openDir} auto scale={0.75}> Pick a directory </Button>
              </Grid>
            </Grid.Container>
            <Spacer />
            <Textarea value={fileTreeTextRef.current} readOnly type="secondary" width="100%" height="250px" />
            <Spacer />
            <Button type="success-light" onClick={backup} scale={0.75}>backup</Button>
            <Spacer />
            <Progress value={fileTreeCountRef.current} max={maxFileTreeCount} />
            <Spacer />
          </Card>
        </Grid>
      </Grid.Container>
    </Page>
  )
}