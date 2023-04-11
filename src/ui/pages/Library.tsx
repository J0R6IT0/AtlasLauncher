import React, { useEffect } from 'react';
import '../styles/Library.css';

import { listen } from '@tauri-apps/api/event';
import toast from 'react-hot-toast';

interface CreateInstanceEvent {
    payload: CreateInstanceEventPayload
}

interface CreateInstanceEventPayload {
    status: string
    message: string
    name: string
}

function Library(): JSX.Element {
    useEffect(() => {
        listen('create_instance', (event: CreateInstanceEvent) => {
            if (event.payload.status === 'Success') {
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
                <div className='instance'>
                </div>
            </div>
        </div>
    );
}

export default Library;
