import React, { useEffect, useRef } from 'react';
import '../styles/ContextMenu.css';
import TrashIcon from '../../assets/icons/trash.svg';
import ToolIcon from '../../assets/icons/tool.svg';
import FolderIcon from '../../assets/icons/folder.svg';
import { invoke } from '@tauri-apps/api';

interface ContextMenuProps {
    target: Element | null
    onClose: () => void
    position: { x: number, y: number }
    updateInstances: () => void
    manageInstance: () => void
}

function ContextMenu(props: ContextMenuProps): JSX.Element {
    const menuRef = useRef<HTMLDivElement>(null);

    const closeMenu = (): void => {
        menuRef.current?.classList.remove('visible');
        setTimeout(() => {
            props.onClose();
        }, 200);
    };

    const handleOutsideClick = (event: MouseEvent): void => {
        const menu = document.querySelector('.context-menu') as HTMLElement;
        if (!menu.contains(event.target as Node)) {
            closeMenu();
        }
    };

    useEffect(() => {
        if (menuRef.current == null) {
            return;
        }
        setTimeout(() => {
            menuRef.current?.classList.add('visible');
        }, 10);
        const rect = menuRef.current.getBoundingClientRect();
        const windowWidth = window.innerWidth;
        const windowHeight = window.innerHeight;

        if (rect.right > windowWidth) {
            const newLeft = props.position.x - rect.width;
            menuRef.current.style.left = `${newLeft}px`;
        }
        if (rect.bottom > windowHeight) {
            const newTop = props.position.y - rect.height;
            menuRef.current.style.top = `${newTop}px`;
        }

        document.addEventListener('click', handleOutsideClick);

        return () => {
            document.removeEventListener('click', handleOutsideClick);
        };
    }, [props.position.x, props.position.y]);

    return (
        <div ref={menuRef} className='context-menu' style={{ left: props.position.x, top: props.position.y }}>
            <div className='context-menu-item clickable' onClick={() => {
                closeMenu();
                props.manageInstance();
            }}>
                <img src={ToolIcon} />
                <span>Manage Instance</span>
            </div>
            <div className='context-menu-item clickable' onClick={() => {
                invoke('open_instance_folder', { name: props.target?.querySelector('span')?.innerText }).then(() => {
                }).catch(e => {});
            }}>
                <img src={FolderIcon} />
                <span>Open Folder</span>
            </div>
            <div className='context-menu-item clickable' onClick={() => {
                closeMenu();
                invoke('remove_instance', { name: props.target?.querySelector('span')?.innerText }).then(() => {
                    props.updateInstances();
                }).catch(e => {});
            }}>
                <img src={TrashIcon} />
                <span>Remove Instance</span>
            </div>
        </div>
    );
}

export default ContextMenu;
