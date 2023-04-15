import React, { useEffect, useState } from 'react';
import SideBar from './ui/components/SideBar';
import NewInstance from './ui/pages/NewInstance';
import Library from './ui/pages/Library';
import AccountSelector from './ui/components/AccountSelector';
import './ui/styles/App.css';
import { appWindow } from '@tauri-apps/api/window';
import MinusIcon from './assets/icons/minus.svg';
import SquareIcon from './assets/icons/square.svg';
import XIcon from './assets/icons/x.svg';
import toast, { Toaster } from 'react-hot-toast';
import BackgroundImage from './assets/images/minecraft-background.webp';
import UserIcon from './assets/icons/user.svg';
import BellIcon from './assets/icons/bell.svg';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

interface CreateInstanceEvent {
    payload: CreateInstanceEventPayload
}

interface CreateInstanceEventPayload {
    status: string
    message: string
    name: string
    version: string
}

export interface InstanceInfo {
    name: string
    version: string
}

let instancesFirstRun: InstanceInfo[] = await invoke('get_instances').catch(e => {}) as InstanceInfo[];

function App(): JSX.Element {
    const [activePage, setActivePage] = useState(2);
    const [accountSelectorActive, setAccountSelectorActive] = useState(false);
    const [instances, setInstances] = useState(instancesFirstRun);

    async function getInstances(): Promise<void> {
        const newInstances = await invoke('get_instances').catch(e => {}) as InstanceInfo[];
        instancesFirstRun = newInstances;
        setInstances(newInstances);
    }

    function accountSelectorHandle(): void {
        setAccountSelectorActive(!accountSelectorActive);
    }

    useEffect(() => {
        listen('create_instance', (event: CreateInstanceEvent) => {
            console.log(Math.random());
            if (event.payload.status === 'Success') {
                getInstances().catch(e => {});
                toast.success(event.payload.message, { id: event.payload.name });
            } else if (event.payload.status === 'Error') {
                toast.error(event.payload.message, { id: event.payload.name });
            } else if (event.payload.status === 'Loading') {
                toast.loading(event.payload.message, { id: event.payload.name });
            } else {
                toast.dismiss(event.payload.name);
            }
        }).catch(e => {});
    }, []);

    return (
        <div className='container'>
            <div className='background'>
                <div className='background-color'></div>
                <img className='background-image' src={BackgroundImage} />
            </div>
            <div data-tauri-drag-region className="titlebar">
                <div className="titlebar-button" id="titlebar-minimize" onClick={() => { appWindow.minimize().catch(e => {}); }}>
                    <img
                        src={MinusIcon}
                        alt="minimize"
                    />
                </div>
                <div className="titlebar-button" id="titlebar-maximize" onClick={() => { appWindow.maximize().catch(e => {}); }}>
                    <img
                        src={SquareIcon}
                        alt="maximize"
                        style={{ height: '0.8rem' }}
                    />
                </div>
                <div className="titlebar-button" id="titlebar-close" onClick={() => { appWindow.close().catch(e => {}); }}>
                    <img src={XIcon} />
                </div>
            </div>
            <SideBar setActivePage={setActivePage} activePage={activePage}/>
            <AccountSelector visible={accountSelectorActive} setVisible={setAccountSelectorActive}/>
            <div className='content'>
                {activePage === 1 && <NewInstance />}
                {activePage === 2 && <Library instances={instances}/>}
            </div>
            <div className='secondary-buttons'>
                <div>
                    <img src={BellIcon} />
                </div>
                <div onClick={accountSelectorHandle} id='accounts-button'>
                    <img src={UserIcon} />
                </div>
            </div>
            <Toaster
                position='bottom-center'
                toastOptions={{
                    success: {
                        duration: 6000,
                        className: 'toast-notification',
                        iconTheme: {
                            primary: 'var(--icons-color-hover)',
                            secondary: 'var(--text-color-primary)'
                        }
                    },
                    error: {
                        duration: 10000,
                        className: 'toast-notification',
                        iconTheme: {
                            primary: 'var(--icons-color-hover)',
                            secondary: 'var(--text-color-primary)'
                        }
                    },
                    loading: {
                        className: 'toast-notification',
                        iconTheme: {
                            primary: 'var(--icons-color-hover)',
                            secondary: 'var(--icons-color)'
                        }
                    }
                }}/>
        </div>
    );
}

export default App;
