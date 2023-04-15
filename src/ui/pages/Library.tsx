import React from 'react';
import '../styles/Library.css';
import { invoke } from '@tauri-apps/api/tauri';
import type { InstanceInfo } from '../../App';
import GrassBlock from '../../assets/images/grass-block.webp';
import InstanceBackground from '../../assets/images/instance-background.webp';
import BoxIcon from '../../assets/icons/box.svg';

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
    );
}

export default Library;
