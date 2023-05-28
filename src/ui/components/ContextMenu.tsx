import React, { useEffect, useRef } from 'react';
import '../styles/ContextMenu.css';
import { invoke } from '@tauri-apps/api';
import mountAnimationHandler from '../../utils/mountAnimationHandler';
import { FolderIcon, ToolIcon, TrashIcon } from '../../assets/icons/Icons';
import { type InstanceInfo } from '../../App';

interface ContextMenuProps {
    target: InstanceInfo | null;
    onClose: () => void;
    position: { x: number; y: number };
    updateInstances: () => void;
    manageInstance: () => void;
}

function ContextMenu(props: ContextMenuProps): JSX.Element {
    const menuRef = useRef<HTMLDivElement>(null);

    const handleClose = (): void => {
        menuRef.current?.classList.remove('visible');
        setTimeout(() => {
            props.onClose();
        }, 200);
    };

    mountAnimationHandler(menuRef, handleClose);

    useEffect(() => {
        if (menuRef.current == null) {
            return;
        }
        const rect = menuRef.current.getBoundingClientRect();
        const windowWidth = window.innerWidth - 16;
        const windowHeight = window.innerHeight - 16;
        if (rect.right > windowWidth) {
            const newLeft = Math.min(
                props.position.x,
                windowWidth - rect.width
            );
            menuRef.current.style.transform = `${newLeft}px`;
        }
        if (rect.bottom > windowHeight) {
            const newTop = Math.min(
                props.position.y,
                windowHeight - rect.height
            );
            menuRef.current.style.top = `${newTop}px`;
        }
    }, [props.position.x, props.position.y]);

    return (
        <div
            ref={menuRef}
            className='context-menu'
            style={{ left: props.position.x, top: props.position.y }}
        >
            <li
                className='context-menu-item clickable hover accent-text-secondary'
                onClick={() => {
                    handleClose();
                    props.manageInstance();
                }}
            >
                <ToolIcon />
                <span>Manage Instance</span>
            </li>
            <li
                className='context-menu-item clickable hover accent-text-secondary'
                onClick={() => {
                    invoke('open_instance_folder', {
                        name: props.target?.name,
                    })
                        .then(() => {})
                        .catch((e) => {});
                }}
            >
                <FolderIcon />
                <span>Open Folder</span>
            </li>
            <li
                className='context-menu-item clickable hover accent-text-secondary'
                onClick={() => {
                    handleClose();
                    invoke('remove_instance', {
                        name: props.target?.name,
                    })
                        .then(() => {
                            props.updateInstances();
                        })
                        .catch((e) => {});
                }}
            >
                <TrashIcon />
                <span>Remove Instance</span>
            </li>
        </div>
    );
}

export default ContextMenu;
