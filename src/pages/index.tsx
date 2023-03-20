import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect } from 'react'

import { Card, ButtonGroup, Tree, Input, Progress, Page, Spacer, Text, Button, Grid } from '@geist-ui/core';

import { Github, Save, Rewind, Database } from '@geist-ui/icons';

const inter = Inter({ subsets: ['latin'] })
//const isClient = typeof window !== 'undefined'

//isClient &&
//  invoke('greet', { name: 'World' }).then(console.log).catch(console.error)



export default function Home() {

  useEffect(() => {
    invoke('greet', { name: 'World' })
      .then(console.log)
      .catch(console.error)
  }, []);

  const files = [{
    type: 'directory',
    name: 'bin',
    files: [{
      type: 'file',
      name: 'cs.js',
    }],
  }, {
    type: 'directory',
    name: 'docs',
    files: [{
      type: 'file',
      name: 'controllers.md',
    }, {
      type: 'file',
      name: 'es6.md',
    }, {
      type: 'file',
      name: 'production.md',
    }, {
      type: 'file',
      name: 'views.md',
    }],
  }]

  return (
    <Page>
      <Grid.Container gap={2} justify="center" >
        <Grid xs={2} direction='column'>
          <Button iconRight={<Save />} auto px={0.6} />
          <Button iconRight={<Rewind />} auto px={0.6} />
          <Button iconRight={<Database />} auto px={0.6} />
        </Grid>
        <Grid xs={22} height="100%">
          <Card shadow width="100%" >
            <Text h5 my={0}>Backup</Text>
            <Spacer h={.5} />
            <Grid.Container gap={2}>
              <Grid >
                <Input readOnly placeholder="Folder path" width="100%" />
              </Grid>
              <Grid >
                <Button auto scale={0.75}> Pick a directory </Button>
              </Grid>
            </Grid.Container>
            <Spacer />

            <Tree value={files} />
            <Spacer />

            <ButtonGroup >
              <Button>First backup</Button>
              <Button>Incremental Backup</Button>

            </ButtonGroup>

            <Spacer />
            <Progress value={45} max={50} />

          </Card>
        </Grid>
      </Grid.Container>


    </Page>
  )
}
