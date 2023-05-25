import React, { useEffect, useState } from 'react';
import SideBar from './ui/components/SideBar';
import NewInstance from './ui/pages/NewInstance';
import Library from './ui/pages/Library';
import AccountSelector from './ui/components/AccountSelector';
import './ui/styles/App.css';
import { appWindow } from '@tauri-apps/api/window';
import toast, { Toaster } from 'react-hot-toast';
import BackgroundImage from './assets/images/minecraft-background.webp';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import DownloadsMenu, {
    type DownloadItemProps,
} from './ui/components/DownloadsMenu';
import Modpacks from './ui/pages/Modpacks';
import {
    HomeIcon,
    MinusIcon,
    SquareIcon,
    XIcon,
    GridIcon,
    PlusIcon,
    PackageIcon,
    SettingsIcon,
    BellIcon,
    DownloadIcon,
    UserIcon,
} from './assets/icons/Icons';

interface StartInstanceEvent {
    payload: StartInstanceEventPayload;
}

interface StartInstanceEventPayload {
    base: BaseEventPayload;
}

interface DownloadEvent {
    payload: DownloadEventPayload;
}

interface DownloadEventPayload {
    base: BaseEventPayload;
    total: number;
    downloaded: number;
    name: string;
}

export interface InstanceInfo {
    name: string;
    modloader: string;
    version: string;
    background: string;
    icon: string;
    version_type: string;
    height: string;
    width: string;
    fullscreen: boolean;
    [key: string]: any;
}

export interface AccountInfo {
    username: string;
    uuid: string;
    active: boolean;
    avatar_64px: string;
}

interface LoginEvent {
    payload: LoginEventPayload;
}

interface LoginEventPayload {
    base: BaseEventPayload;
}

interface BaseEventPayload {
    status: string;
    message: string;
}

interface SecondaryButtonsProps {
    refreshInstances: () => void;
}

export enum Pages {
    Home,
    Library,
    New,
    Modpacks,
    Settings,
}

export const pages = [
    {
        page: Pages.Home,
        icon: HomeIcon,
        name: 'Home',
        desc: 'nothing here yet',
    },
    {
        page: Pages.Library,
        icon: GridIcon,
        name: 'Library',
        desc: 'Your Minecraft worlds are awaiting',
    },
    {
        page: Pages.New,
        icon: PlusIcon,
        name: 'New Instance',
        desc: 'Add a new instance to your library',
    },
    {
        page: Pages.Modpacks,
        icon: PackageIcon,
        name: 'Modpacks',
        desc: 'Ready-to-play modpacks',
    },
    {
        page: Pages.Settings,
        icon: SettingsIcon,
        name: 'Settings',
        desc: 'nothing here yet',
    },
];

function SecondaryButtons(props: SecondaryButtonsProps): JSX.Element {
    const [accountSelectorActive, setAccountSelectorActive] = useState(false);
    const [downloadsActive, setDownloadsActive] = useState(false);
    const [downloads, setDownloads] = useState<DownloadItemProps[]>([]);
    const [accounts, setAccounts] = useState<AccountInfo[]>([]);
    const [activeAccount, setActiveAccount] = useState<AccountInfo | null>(
        null
    );

    async function getAccounts(): Promise<void> {
        const accounts = (await invoke('get_accounts').catch(
            (e) => {}
        )) as AccountInfo[];
        setAccounts(accounts);
        const activeAccount = accounts.filter((acc) => acc.active)[0];
        if (activeAccount !== null && activeAccount !== undefined) {
            setActiveAccount(activeAccount);
        } else {
            setActiveAccount(null);
        }
    }

    useEffect(() => {
        listen('auth', (event: LoginEvent) => {
            if (event.payload.base.status === 'Success') {
                getAccounts().catch((e) => {});
                toast.success(event.payload.base.message, {
                    id: 'currentLoginNotification',
                });
            } else if (event.payload.base.status === 'Error') {
                toast.error(event.payload.base.message, {
                    id: 'currentLoginNotification',
                });
            } else if (event.payload.base.status === 'Loading') {
                toast.loading(event.payload.base.message, {
                    id: 'currentLoginNotification',
                });
            } else {
                toast.dismiss('currentLoginNotification');
            }
        }).catch((e) => {});
        listen('download', (event: DownloadEvent) => {
            if (event.payload.base.status === 'Success') {
                setDownloads((prevDownloads) => [
                    ...prevDownloads.filter(
                        (download) => download.name !== event.payload.name
                    ),
                ]);
                props.refreshInstances();
            } else if (event.payload.base.status === 'Loading') {
                setDownloads((prevDownloads) => [
                    ...prevDownloads.filter(
                        (download) => download.name !== event.payload.name
                    ),
                    {
                        name: event.payload.name,
                        downloaded: event.payload.downloaded,
                        total: event.payload.total,
                        step: event.payload.base.message,
                    },
                ]);
            } else if (event.payload.base.status === 'Update') {
                setDownloads((prevDownloads) => {
                    return prevDownloads.map((download) => {
                        if (download.name === event.payload.name) {
                            return {
                                ...download,
                                downloaded:
                                    download.downloaded +
                                    event.payload.downloaded,
                            };
                        }
                        return download;
                    });
                });
            }
        }).catch((e) => {});
        getAccounts().catch((e) => {});
    }, []);

    return (
        <div className='secondary-buttons'>
            <div className='secondary-button clickable hover accent-text-primary'>
                <BellIcon />
            </div>
            <div
                className='secondary-button clickable hover accent-text-primary'
                onClick={() => {
                    if (!downloadsActive) setDownloadsActive(true);
                }}
            >
                <DownloadIcon />
                {downloads.length > 0 && (
                    <div className='active-downloads-notification' />
                )}
            </div>
            <div
                className={`secondary-button clickable hover ${
                    activeAccount === null ? 'accent-text-primary' : ''
                }`}
                onClick={() => {
                    if (!accountSelectorActive) setAccountSelectorActive(true);
                }}
            >
                {activeAccount === null ? (
                    <UserIcon />
                ) : (
                    <img
                        src={`data:image/png;base64,${activeAccount.avatar_64px}`}
                    />
                )}
            </div>
            {accountSelectorActive && (
                <AccountSelector
                    onClose={() => {
                        setAccountSelectorActive(false);
                    }}
                    accounts={accounts}
                    updateAccounts={() => {
                        getAccounts().catch((e) => {});
                    }}
                />
            )}
            {downloadsActive && (
                <DownloadsMenu
                    onClose={() => {
                        setDownloadsActive(false);
                    }}
                    items={downloads}
                />
            )}
        </div>
    );
}

function App(): JSX.Element {
    const [activePage, setActivePage] = useState(Pages.Library);
    const [instances, setInstances] = useState<InstanceInfo[]>([]);

    async function getInstances(): Promise<void> {
        const newInstances = (await invoke('get_instances').catch(
            (e) => {}
        )) as InstanceInfo[];
        setInstances(newInstances);
    }

    function contextMenuHandler(event: Event): void {
        event.preventDefault();
    }

    useEffect(() => {
        listen('start_instance', (event: StartInstanceEvent) => {
            if (event.payload.base.status === 'Success') {
                toast.success(event.payload.base.message, {
                    id: 'startInstance',
                });
            } else if (event.payload.base.status === 'Error') {
                toast.error(event.payload.base.message, {
                    id: 'startInstance',
                });
            } else {
                toast.loading(event.payload.base.message, {
                    id: 'startInstance',
                });
            }
        }).catch((e) => {});
        getInstances().catch((e) => {});

        document.addEventListener('contextmenu', contextMenuHandler);
        return () => {
            document.removeEventListener('contextmenu', contextMenuHandler);
        };
    }, []);

    return (
        <React.Fragment>
            <img
                className='background background-image'
                src={BackgroundImage}
            />
            <div className='background background-color' />
            <div data-tauri-drag-region className='titlebar'>
                <div
                    className='titlebar-button clickable hover accent-icons'
                    onClick={() => {
                        appWindow.minimize().catch((e) => {});
                    }}
                >
                    <MinusIcon />
                </div>
                <div
                    className='titlebar-button clickable hover accent-icons'
                    onClick={() => {
                        appWindow.maximize().catch((e) => {});
                    }}
                >
                    <SquareIcon />
                </div>
                <div
                    className='titlebar-button clickable hover accent-icons'
                    onClick={() => {
                        appWindow.close().catch((e) => {});
                    }}
                >
                    <XIcon />
                </div>
            </div>
            <SideBar setActivePage={setActivePage} activePage={activePage} />
            <div className='page'>
                <div className='page-info'>
                    <span className='page-title'>{pages[activePage].name}</span>
                    <span>{pages[activePage].desc}</span>
                </div>
                <div className='page-content'>
                    {activePage === Pages.New && (
                        <NewInstance
                            goToLibrary={() => {
                                setActivePage(Pages.Library);
                            }}
                        />
                    )}
                    {activePage === Pages.Library && (
                        <Library
                            instances={instances}
                            updateInstances={() => {
                                getInstances().catch((e) => {});
                            }}
                        />
                    )}
                    {activePage === Pages.Modpacks && <Modpacks />}
                </div>
            </div>
            <SecondaryButtons
                refreshInstances={() => {
                    getInstances().catch((e) => {});
                }}
            />
            <Toaster
                position='bottom-center'
                toastOptions={{
                    success: {
                        duration: 6000,
                        className: 'toast-notification',
                        iconTheme: {
                            primary: 'var(--icons-color-hover)',
                            secondary: 'var(--text-color-primary)',
                        },
                    },
                    error: {
                        duration: 10000,
                        className: 'toast-notification',
                        iconTheme: {
                            primary: 'var(--icons-color-hover)',
                            secondary: 'var(--text-color-primary)',
                        },
                    },
                    loading: {
                        className: 'toast-notification',
                        iconTheme: {
                            primary: 'var(--icons-color-hover)',
                            secondary: 'var(--icons-color)',
                        },
                    },
                }}
            />
        </React.Fragment>
    );
}

export default App;
