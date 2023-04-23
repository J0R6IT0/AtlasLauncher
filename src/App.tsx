import React, { useEffect, useRef, useState } from 'react';
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
    base: BaseEventPayload
    name: string
}

interface StartInstanceEvent {
    payload: StartInstanceEventPayload
}

interface StartInstanceEventPayload {
    base: BaseEventPayload
}

export interface InstanceInfo {
    name: string
    version: string
    background: string
}

export interface AccountInfo {
    username: string
    uuid: string
    active: boolean
}

interface LoginEvent {
    payload: LoginEventPayload
}

interface LoginEventPayload {
    base: BaseEventPayload
}

interface BaseEventPayload {
    status: string
    message: string
}

function SecondaryButtons(): JSX.Element {
    const [accountSelectorActive, setAccountSelectorActive] = useState(false);
    const [accounts, setAccounts] = useState<AccountInfo[]>([]);

    async function getAccounts(): Promise<void> {
        const accounts = await invoke('get_accounts').catch(e => {}) as AccountInfo[];
        setAccounts(accounts);
        const button = accountButtonRef.current;
        const accountsIcon = button?.querySelector('img');
        const activeAccount = accounts.filter(acc => acc.active)[0];
        if (activeAccount !== null && activeAccount !== undefined) {
            accountsIcon?.setAttribute('src', `https://crafatar.com/avatars/${activeAccount.uuid}?overlay`);
            button?.classList.add('active-user');
        } else {
            accountsIcon?.setAttribute('src', UserIcon);
            button?.classList.remove('active-user');
        }
    }

    const accountButtonRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        listen('auth', (event: LoginEvent) => {
            if (event.payload.base.status === 'Success') {
                getAccounts().catch(e => {});
                toast.success(event.payload.base.message, { id: 'currentLoginNotification' });
            } else if (event.payload.base.status === 'Error') {
                toast.error(event.payload.base.message, { id: 'currentLoginNotification' });
            } else if (event.payload.base.status === 'Loading') {
                console.log('e');
                toast.loading(event.payload.base.message, { id: 'currentLoginNotification' });
            } else {
                toast.dismiss('currentLoginNotification');
            }
        }).catch(e => {});

        getAccounts().catch(e => {});
    }, []);

    return (
        <div className='secondary-buttons'>
            <div className='secondary-button clickable'>
                <img src={BellIcon} />
            </div>
            <div className='secondary-button clickable' onClick={() => {
                if (!accountSelectorActive) setAccountSelectorActive(true);
            }} ref={accountButtonRef}>
                <img src={UserIcon} />
            </div>
            {accountSelectorActive && <AccountSelector onClose={() => { setAccountSelectorActive(false); }} accounts={accounts} updateAccounts={() => {
                getAccounts().catch(e => {});
            }}/>}
        </div>
    );
}

function App(): JSX.Element {
    const [activePage, setActivePage] = useState(2);
    const [instances, setInstances] = useState<InstanceInfo[]>([]);

    async function getInstances(): Promise<void> {
        const newInstances = await invoke('get_instances').catch(e => {}) as InstanceInfo[];
        setInstances(newInstances);
    }

    useEffect(() => {
        listen('create_instance', (event: CreateInstanceEvent) => {
            if (event.payload.base.status === 'Success') {
                getInstances().catch(e => {});
                toast.success(event.payload.base.message, { id: event.payload.name });
            } else if (event.payload.base.status === 'Error') {
                toast.error(event.payload.base.message, { id: event.payload.name });
            } else if (event.payload.base.status === 'Loading') {
                toast.loading(event.payload.base.message, { id: event.payload.name });
            } else {
                toast.dismiss(event.payload.name);
            }
        }).catch(e => {});
        listen('start_instance', (event: StartInstanceEvent) => {
            if (event.payload.base.status === 'Success') {
                toast.success(event.payload.base.message, { id: 'startInstance' });
            } else if (event.payload.base.status === 'Error') {
                toast.error(event.payload.base.message, { id: 'startInstance' });
            } else {
                toast.loading(event.payload.base.message, { id: 'startInstance' });
            }
        }).catch(e => {});
        getInstances().catch(e => {});

        function contextMenuHandler(event: Event): void {
            // event.preventDefault();
        }
        document.addEventListener('contextmenu', contextMenuHandler);
        return () => {
            document.removeEventListener('contextmenu', contextMenuHandler);
        };
    }, []);

    return (
        <div className='container'>
            <div className='background'>
                <div className='background-color'></div>
                <img className='background-image' src={BackgroundImage} />
            </div>
            <div data-tauri-drag-region className="titlebar">
                <div className="titlebar-button clickable" onClick={() => { appWindow.minimize().catch(e => {}); }}>
                    <img src={MinusIcon} />
                </div>
                <div className="titlebar-button clickable" onClick={() => { appWindow.maximize().catch(e => {}); }}>
                    <img src={SquareIcon} style={{ height: '0.8rem' }} />
                </div>
                <div className="titlebar-button clickable" onClick={() => { appWindow.close().catch(e => {}); }}>
                    <img src={XIcon} />
                </div>
            </div>
            <SideBar setActivePage={setActivePage} activePage={activePage}/>
            <div className='content'>
                {activePage === 1 && <NewInstance goToLibrary={() => {
                    setActivePage(2);
                }}/>}
                {activePage === 2 && <Library instances={instances} updateInstances={() => {
                    getInstances().catch(e => {});
                }}/>}
            </div>
            <SecondaryButtons />
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
