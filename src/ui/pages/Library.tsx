import React, { useState } from 'react';
import '../styles/Library.css';
import { invoke, convertFileSrc } from '@tauri-apps/api/tauri';
import type { InstanceInfo } from '../../App';
import DefaultBackground1 from '../../assets/images/default-background-1.webp';
import DefaultBackground2 from '../../assets/images/default-background-2.webp';
import DefaultIcon1 from '../../assets/images/default-icon-1.webp';
import DefaultIcon2 from '../../assets/images/default-icon-5.webp';
import DefaultIcon3 from '../../assets/images/default-icon-4.webp';
import DefaultIcon4 from '../../assets/images/default-icon-2.webp';
import DefaultIcon5 from '../../assets/images/default-icon-3.webp';
import DefaultIcon6 from '../../assets/images/default-icon-6.webp';
import BoxIcon from '../../assets/icons/box.svg';
import FabricIcon from '../../assets/icons/fabric.svg';
import ForgeIcon from '../../assets/icons/forge.svg';
import { toast } from 'react-hot-toast';
import ContextMenu from '../components/ContextMenu';
import ManageInstance from '../components/ManageInstance';
import BaseModal from '../components/BaseModal';

export const defaultBackgrounds = [DefaultBackground1, DefaultBackground2];
export const defaultIcons = [DefaultIcon1, DefaultIcon2, DefaultIcon3, DefaultIcon4, DefaultIcon5, DefaultIcon6];

interface LibraryProps {
    instances: InstanceInfo[]
    updateInstances: () => void
}

interface InstanceProps {
    element: InstanceInfo
    handleContextMenu: (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => void
    setShowRetryModal: (name: string) => void
    onClick: () => void
}

function Library(props: LibraryProps): JSX.Element {
    const [showContextMenu, setShowContextMenu] = useState(false);
    const [contextMenuTarget, setShowContextMenuTarget] = useState<Element | null>(null);
    const [contextMenuPosition, setContextMenuPosition] = useState<{ x: number, y: number }>({ x: 0, y: 0 });
    const [showManageInstance, setShowManageInstance] = useState(false);
    const [showRetryModal, setShowRetryModal] = useState<string | null>(null);

    const handleContextMenu = (event: React.MouseEvent<HTMLDivElement, MouseEvent>): void => {
        setShowContextMenu(true);
        setShowContextMenuTarget(event.currentTarget);
        setContextMenuPosition({ x: event.clientX, y: event.clientY });
    };

    return (
        <div className='library'>
            <div className='page-info'>
                <span className='page-title'>Library</span>
                <span>Your Minecraft worlds are awaiting</span>
            </div>
            <div className='instances'>
                <div className='grid'>
                    {props.instances.map((element, key) => <Instance key={key} element={element} handleContextMenu={handleContextMenu} setShowRetryModal={(name) => { setShowRetryModal(name); }} onClick={() => {
                        if (element.version.startsWith('rd-')) {
                            setShowRetryModal(element.name);
                        } else {
                            invoke('launch_instance', { name: element.name }).catch(e => {});
                            toast.loading(`Launching ${element.name}`, { id: 'startInstance' });
                        }
                    }}/>)}
                </div>
            </div>
            {showRetryModal !== null && <BaseModal title='IMPORTANT' description='Old Pre-Classic versions usually crash multiple times until they finally launch. Atlas will attempt to launch the instance multiple times. You may see a window popping up multiple times.' onClose={() => {
                setShowRetryModal(null);
            }} onAccept={() => {
                invoke('launch_instance', { name: showRetryModal }).catch(e => {});
                setShowRetryModal(null);
            }}/>}
            {showContextMenu && (
                <ContextMenu target={contextMenuTarget} onClose={() => { setShowContextMenu(false); }} position={contextMenuPosition} updateInstances={props.updateInstances} manageInstance={() => { setShowManageInstance(true); }} />
            )}
            {showManageInstance && <ManageInstance onClose={() => { setShowManageInstance(false); }} target={contextMenuTarget} updateInstances={props.updateInstances}/>}
        </div>
    );
}

export default Library;

function Instance(props: InstanceProps): JSX.Element {
    return (
        <div className='instance' onClick={props.onClick}
            onContextMenu={props.handleContextMenu}>
            <div className='instance-content'>
                <img className='instance-background' src={!props.element.background.startsWith('default') ? convertFileSrc(props.element.background) : defaultBackgrounds[parseInt(props.element.background.replace('default', ''))]} />
                <div className='instance-info'>
                    <span><img src={!props.element.icon.startsWith('default') ? convertFileSrc(props.element.icon) : defaultIcons[parseInt(props.element.icon.replace('default', ''))]} />{props.element.name}</span>
                    <div className='instance-version'><span>{props.element.version}</span></div>
                    <div className='instance-type fabric'><img className={props.element.modloader.startsWith('fabric') ? 'fabric-icon' : ''} src={props.element.modloader.startsWith('forge') ? ForgeIcon : props.element.modloader.startsWith('fabric') ? FabricIcon : BoxIcon} /></div>
                </div>
            </div>
        </div>

    );
};

export { Instance };
