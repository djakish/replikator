
import { Divider, Modal } from '@geist-ui/core'
import { Maximize, Minus, X } from '@geist-ui/icons'

import { appWindow } from '@tauri-apps/api/window'
import { useState } from 'react'

export default function Controls() {
    const [state, setState] = useState(false)
    const handler = () => setState(true)
    const closeHandler = () => {
        setState(false)
    }
    async function onMinimize() {
        appWindow.minimize()
    }

    async function onMaximize() {
        appWindow.toggleMaximize()
    }

    async function onClose() {
        appWindow.close()
    }

    return (
        <div data-tauri-drag-region className="titlebar">
            <div className="titlebar-button" id="titlebar-minimize">
                <Minus size={20} onClick={onMinimize} />
            </div>
            {/* <div className="titlebar-button" id="titlebar-maximize">
                <Maximize size={20} onClick={onMaximize} />
            </div> */}
            <div className="titlebar-button" id="titlebar-close">
                <X size={20} onClick={handler} />
            </div>
            <Modal visible={state} onClose={closeHandler}>
                <Modal.Title>Are you sure you want to exit?</Modal.Title>
                <Modal.Action passive onClick={onClose}>Yes</Modal.Action>
                <Modal.Action passive onClick={() => setState(false)}>No</Modal.Action>
            </Modal>

        </div>
    )
}

