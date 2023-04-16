
import { invoke } from '@tauri-apps/api/tauri'

// my imports
import { useCurrentState, Card, Textarea, Input, Progress, Page, Spacer, Text, Button, Grid } from '@geist-ui/core';
import { open } from '@tauri-apps/api/dialog';
import { useState } from 'react';
import RouterButtons from '@/components/RouterButtons'
import { appWindow } from "@tauri-apps/api/window";
import Controls from './Controls';

interface Payload {
    message: string;
}
type ProgressHandler = (progress: number, total: number) => void;


export default function DynamicDatabase() {
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
            await setMaxFileTreeCount(file_tree_result[1])
            await setFileTreeCount(0)
        }
        setFileTreeLoading(false)
    }



    async function restore() {
        setIsLoading(true)
        const selected = open({
            directory: true,
            multiple: false,
            defaultPath: '~/',
        }); if (selected === null) {
            // user cancelled the selection
        } else {
            await setFileTreeText('')
            await listenToEventIfNeeded("compress://progress")
            await invoke('decompress_files', { input: path, output: await selected })
        }
        setIsLoading(false)
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
            })
            .then(() => {
                listening = true;
            });
    }

    return (
        <Page render='effect-seo' dotBackdrop={true} dotSize="2px">
            <Controls />
            <Grid.Container gap={2} justify="flex-start">
                <RouterButtons />
                <Grid sm={22} >
                    <Card hoverable shadow width="100%"  >
                        <Text h4 my={0}>My Backups</Text>
                        <Spacer/>
                        <Grid.Container gap={2}>
                            <Grid xs={12} justify="center">
                                <Card width="100%">
                                    <Text h4 my={0}>Backup #1</Text>
                                    <Text>Modern and minimalist React UI library.</Text>
                                    <Card.Footer>
                                    </Card.Footer>
                                </Card>
                            </Grid>
                            <Grid xs={12} justify="center">
                                <Card width="100%">
                                    <Text h4 my={0}>Geist UI React</Text>
                                    <Text>Modern and minimalist React UI library.</Text>
                                    <Card.Footer>
                                    </Card.Footer>
                                </Card>
                            </Grid>
                            <Grid xs={12} justify="center">
                                <Card width="100%">
                                    <Text h4 my={0}>Geist UI React</Text>
                                    <Text>Modern and minimalist React UI library.</Text>
                                    <Card.Footer>
                                    </Card.Footer>
                                </Card>
                            </Grid>
                            <Grid xs={12} justify="center">
                                <Card width="100%">
                                    <Text h4 my={0}>Geist UI React</Text>
                                    <Text>Modern and minimalist React UI library.</Text>
                                    <Card.Footer>
                                    </Card.Footer>
                                </Card>
                            </Grid>
                            <Grid xs={12} justify="center">
                                <Card width="100%">
                                    <Text h4 my={0}>Geist UI React</Text>
                                    <Text>Modern and minimalist React UI library.</Text>
                                    <Card.Footer>
                                    </Card.Footer>
                                </Card>
                            </Grid>
                            <Grid xs={12} justify="center">
                                <Card width="100%">
                                    <Text h4 my={0}>Geist UI React</Text>
                                    <Text>Modern and minimalist React UI library.</Text>
                                    <Card.Footer>
                                    </Card.Footer>
                                </Card>
                            </Grid>
                        </Grid.Container>
                    </Card>
                </Grid>
            </Grid.Container>
        </Page>
    )
}
