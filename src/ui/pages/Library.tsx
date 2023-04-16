import React, { useState } from 'react';
import '../styles/Library.css';
import { invoke } from '@tauri-apps/api/tauri';
import type { InstanceInfo } from '../../App';
import GrassBlock from '../../assets/images/grass-block.webp';
import InstanceBackground from '../../assets/images/instance-background.webp';
import BoxIcon from '../../assets/icons/box.svg';
import { toast } from 'react-hot-toast';
import ContextMenu from '../components/ContextMenu';

interface LibraryProps {
    instances: InstanceInfo[]
    updateInstances: () => void
}

function Library(props: LibraryProps): JSX.Element {
    const [showContextMenu, setShowContextMenu] = useState(false);
    const [contextMenuTarget, setShowContextMenuTarget] = useState<Element | null>(null);
    const [contextMenuPosition, setContextMenuPosition] = useState<{ x: number, y: number }>({ x: 0, y: 0 });

    const handleContextMenu = (event: React.MouseEvent<HTMLDivElement, MouseEvent>): void => {
        setShowContextMenu(true);
        setShowContextMenuTarget(event.currentTarget);
        setContextMenuPosition({ x: event.clientX, y: event.clientY });
    };

    return (
        <div className='library'>
            <div className='library-info'>
                <span className='library-title'>Library</span>
                <span>Your Minecraft worlds are awaiting</span>
            </div>
            <div className='instances'>
                <div className='grid'>
                    {props.instances.map((element, key) => <div key={key} className='instance' onClick={() => {
                        invoke('launch_instance', { name: element.name }).catch(e => {});
                        toast.loading(`Launching ${element.name}`, { id: 'startInstance' });
                    }}
                    onContextMenu={handleContextMenu}>
                        <div className='instance-content'>
                            <img className='instance-background' src={InstanceBackground} />
                            <div className='instance-info'>
                                <span><img src={GrassBlock} />{element.name}</span>
                                <div className='instance-version'><span>{element.version}</span></div>
                                <div className='instance-type'><img src={BoxIcon} /></div>
                            </div>
                        </div>
                    </div>)}
                </div>
            </div>
            {showContextMenu && (
                <ContextMenu target={contextMenuTarget} onClose={() => { setShowContextMenu(false); }} position={contextMenuPosition} updateInstances={props.updateInstances}/>
            )}
        </div>
    );
}

export default Library;
