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

const inter = Inter({ subsets: ['latin'] })
//const isClient = typeof window !== 'undefined'

//isClient &&
//  invoke('greet', { name: 'World' }).then(console.log).catch(console.error)

interface FileTreeValue {
  type: 'directory' | 'file';
  name: string;
  extra?: string;
  files?: Array<FileTreeValue>;
};

export default function Home() {
  const [path, setPath] = useState<string | undefined>('');
  const [fileTreeText, setFileTreeText] = useState<string | undefined>('');

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
      let file_tree: string = await invoke('get_files_tree', { path: await selected })
      await setFileTreeText(file_tree)
    }
    setFileTreeLoading(false)
  }


  async function restore() {
    setFileTreeLoading(false)
    const selected = open({
      directory: true,
      multiple: false,
      defaultPath: '~/',
    }); if (selected === null) {
      // user cancelled the selection
    } else {
      await invoke('decompress_files', { input: path, output: await selected })
    }
  }

  return (
    <Page >
      <Grid.Container >
        <RouterButtons/>
        <Grid sm={22} height="100%" >
          <Card shadow width="100%" >
            <Text h4 my={0}>Restore a backup</Text>
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
            <Textarea value={fileTreeText} readOnly type="secondary" width="100%" height="250px" />
            <Spacer />
            <Button type="success-light" onClick={restore} scale={0.75}>restore</Button>
            <Spacer />
            <Progress value={0} max={50} />
            <Spacer />
          </Card>
        </Grid>
      </Grid.Container>
    </Page>
  )
}
