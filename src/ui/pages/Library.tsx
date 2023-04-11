import React, { useEffect, useState } from 'react';
import '../styles/Library.css';

import { listen } from '@tauri-apps/api/event';
import toast from 'react-hot-toast';
import { invoke } from '@tauri-apps/api/tauri';

interface CreateInstanceEvent {
    payload: CreateInstanceEventPayload
}

interface CreateInstanceEventPayload {
    status: string
    message: string
    name: string
    version: string
}

interface InstanceInfo {
    name: string
    version: string

}

let instancesFirstRun: InstanceInfo[] = await invoke('get_instances').catch(e => {}) as InstanceInfo[];

function Library(): JSX.Element {
    const [instances, setInstances] = useState(instancesFirstRun);

    async function getInstances(): Promise<void> {
        const newInstances = await invoke('get_instances').catch(e => {}) as InstanceInfo[];
        instancesFirstRun = newInstances;
        setInstances(newInstances);
    }

    useEffect(() => {
        listen('create_instance', (event: CreateInstanceEvent) => {
            if (event.payload.status === 'Success') {
                getInstances().catch(e => {});
                toast.success(event.payload.message, {
                    id: event.payload.name,
                    duration: 6000,
                    position: 'bottom-center',
                    iconTheme: {
                        primary: 'var(--icons-color)',
                        secondary: 'var(--icons-color-hover)'
                    }
                });
            } else if (event.payload.status === 'Error') {
                toast.error(event.payload.message, {
                    id: event.payload.name,
                    duration: 10000,
                    position: 'bottom-center',
                    iconTheme: {
                        primary: 'var(--icons-color)',
                        secondary: 'var(--icons-color-hover)'
                    }
                });
            } else if (event.payload.status === 'Loading') {
                toast.loading(event.payload.message, {
                    id: event.payload.name,
                    position: 'bottom-center',
                    className: 'toast-notification',
                    iconTheme: {
                        primary: 'var(--icons-color-hover)',
                        secondary: 'var(--icons-color)'
                    }
                });
            } else {
                toast.dismiss(event.payload.name);
            }
        }).catch(e => {});
    });

    return (
        <div className='library'>
            <div className='library-info'>
                <span className='library-title'>Library</span>
                <span>Your Minecraft worlds are awaiting</span>
            </div>
            <div className='instances'>
                {instances.map((element, key) => <div key={key} className='instance' onClick={() => {
                    invoke('launch_instance', { name: element.name }).catch(e => {});
                }}>
                    <span>{element.name} - {element.version}</span>
                </div>)}

            </div>
        </div>
    );
}

export default Library;
