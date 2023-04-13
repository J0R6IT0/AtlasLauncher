import React from 'react';
import '../styles/Library.css';
import { invoke } from '@tauri-apps/api/tauri';
import type { InstanceInfo } from '../../App';

interface LibraryProps {
    instances: InstanceInfo[]
}

function Library(props: LibraryProps): JSX.Element {
    return (
        <div className='library'>
            <div className='library-info'>
                <span className='library-title'>Library</span>
                <span>Your Minecraft worlds are awaiting</span>
            </div>
            <div className='instances'>
                {props.instances.map((element, key) => <div key={key} className='instance' onClick={() => {
                    invoke('launch_instance', { name: element.name }).catch(e => {});
                }}>
                    <span>{element.name} - {element.version}</span>
                </div>)}

            </div>
        </div>
    );
}

export default Library;
