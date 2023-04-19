
import { invoke } from '@tauri-apps/api/tauri'

// my imports
import { useCurrentState, Modal, Card, Textarea, Input, Progress, Page, Spacer, Text, Button, Grid, Table, useModal } from '@geist-ui/core';
import { open } from '@tauri-apps/api/dialog';
import { useState } from 'react';
import RouterButtons from '@/components/RouterButtons'
import { appWindow } from "@tauri-apps/api/window";
import Controls from './Controls';
import React from 'react';

interface Payload {
    message: string;
}
type ProgressHandler = (progress: number, total: number) => void;


export default function DynamicDatabase() {
    const { setVisible, bindings } = useModal()

    const dataSource = [
        { name: <Text b>bold</Text>, date: 'April 18 16:16', operation: '', delete: '' },
        { name: <Text b>bold</Text>, date: 'April 18 16:16', operation: '', delete: '' },
        { name: <Text b>bold</Text>, date: 'April 18 16:16', operation: '', delete: '' },
        { name: <Text b>bold</Text>, date: 'April 18 16:16', operation: '', delete: '' },
        { name: <Text b>bold</Text>, date: 'April 18 16:16', operation: '', delete: '' },
        { name: <Text b>bold</Text>, date: 'April 18 16:16', operation: '', delete: '' },
        { name: <Text b>bold</Text>, date: 'April 18 16:16', operation: '', delete: '' },
    ]
    const [data, setData] = React.useState(dataSource)


    const renderAction = (value: any, rowData: any, index: number) => {
        const removeHandler = () => {
            setData(last => last.filter((_, dataIndex) => dataIndex !== index))
        }
        return (
            <Button type="success-light" auto scale={1 / 3} font="12px" onClick={removeHandler}>Backup</Button>
        )
    }

    const deleteAction = (value: any, rowData: any, index: number) => {
        const removeHandler = () => {
            setData(last => last.filter((_, dataIndex) => dataIndex !== index))
        }
        return (
            <Button type="error" auto scale={1 / 3} font="12px" onClick={removeHandler}>Remove</Button>
        )
    }

    return (
        <Page render='effect-seo'>
            <Controls />
            <Grid.Container gap={2} justify="flex-start">
                <RouterButtons />
                <Grid sm={22} >
                    <Card hoverable shadow width="100%"  >
                        <Text h4 my={0}>My Backups</Text>
                        <Spacer />
                        <Button auto onClick={() => setVisible(true)} type="success-light" scale={0.75}>create a new backup</Button>
                        <Modal width="35rem" {...bindings}>
                            <Modal.Title>Add a new backup</Modal.Title>
                            <Modal.Content>
                                <Grid.Container gap={2} justify="space-evenly">
                                    <Grid md={17}><Input width="100%" /></Grid>
                                    <Grid md={6}><Button   auto  scale={0.75}>Set Input</Button></Grid>
                                    <Grid md={17}><Input width="100%" /></Grid>
                                    <Grid md={6}><Button auto scale={0.75}>Set Output</Button></Grid>
                                </Grid.Container>
                            </Modal.Content>
                        </Modal>

                        <Spacer />
                        <Table data={data} >
                            <Table.Column prop="name" label="backup name" />
                            <Table.Column prop="date" label="last backup" />
                            <Table.Column prop="operation" width={50} render={renderAction} />
                            <Table.Column prop="delete" label="" width={50} render={deleteAction} />
                        </Table>
                        <Spacer />
                    </Card>
                </Grid>
            </Grid.Container>
        </Page>
    )
}
