import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useState } from 'react';
import UserPlus from '../../assets/icons/user-plus.svg';
import TrashIcon from '../../assets/icons/trash.svg';
import { listen } from '@tauri-apps/api/event';
import toast from 'react-hot-toast';
import UserIcon from '../../assets/icons/user.svg';

import '../styles/AccountSelector.css';

interface AccountInfo {
    username: string
    uuid: string
}

interface LoginEvent {
    payload: LoginEventPayload
}

interface LoginEventPayload {
    status: string
    message: string
}

interface AccountSelectorProps {
    visible: boolean
    setVisible: (visible: boolean) => void
}

function AccountSelector(props: AccountSelectorProps): JSX.Element {
    const [accounts, setAccounts] = useState<AccountInfo[]>([]);
    const [activeAccount, setActiveAccount] = useState('');

    async function getAccounts(): Promise<void> {
        const accounts = await invoke('get_accounts').catch(e => {}) as AccountInfo[];
        const activeAccount = await invoke('get_active_account').catch(e => {}) as string;
        setAccounts(accounts);
        setActiveAccount(activeAccount);
        const button = document.getElementById('accounts-button');
        if (activeAccount !== null && activeAccount.length > 1) {
            const user = accounts.find(user => user.uuid === activeAccount);
            if (user !== null && user !== undefined) {
                const accountsIcon = document.querySelector('#accounts-button img');
                accountsIcon?.setAttribute('src', `https://crafatar.com/avatars/${activeAccount}?overlay`);
                button?.classList.add('active-user');

                const usernameSpan = document.querySelector('#accounts-button span');
                if (usernameSpan !== null) usernameSpan.textContent = user.username;
            }
        }
    }

    useEffect(() => {
        const element = document.getElementById('account-selector');
        const button = document.getElementById('accounts-button');

        listen('auth', (event: LoginEvent) => {
            if (event.payload.status === 'Success') {
                getAccounts().catch(e => {});
                toast.success(event.payload.message, { id: 'currentLoginNotification' });
            } else if (event.payload.status === 'Error') {
                toast.error(event.payload.message, { id: 'currentLoginNotification' });
            } else if (event.payload.status === 'Loading') {
                toast.loading(event.payload.message, { id: 'currentLoginNotification' });
            } else {
                toast.dismiss('currentLoginNotification');
            }
        }).catch(e => {});

        getAccounts().catch(e => {});

        function clickHandler(event: Event): void {
            if (element !== null && button !== null) {
                if (!element.contains(event.target as Node) && !button.contains(event.target as Node) && element.classList.contains('visible')) {
                    props.setVisible(false);
                }
            }
        }

        document.addEventListener('click', clickHandler);
        return () => {
            document.removeEventListener('click', clickHandler);
        };
    }, []);

    return (
        <div className={`account-selector ${props.visible ? 'visible' : ''}`} id='account-selector'>
            {accounts.map((element, index) => <div key={index} className={`account-items ${activeAccount === element.uuid ? 'active' : ''}`}>
                <div
                    onClick={() => {
                        invoke('set_active_account', { uuid: element.uuid }).catch(e => {});
                        setActiveAccount(element.uuid);
                    }}>
                    <img src={`https://crafatar.com/avatars/${element.uuid}?overlay`} alt="" />
                    <span>{element.username}
                        <span id='active-account-label'>{activeAccount === element.uuid ? '\nActive' : ''}</span>
                    </span>
                </div>
                <img onClick={() => {
                    invoke('remove_account', { uuid: element.uuid }).catch(e => {});
                    if (activeAccount === element.uuid) {
                        setActiveAccount('');
                        const accountsIcon = document.querySelector('#accounts-button img');
                        accountsIcon?.setAttribute('src', UserIcon);
                        const button = document.getElementById('accounts-button');
                        button?.classList.remove('active-user');
                    }
                    setAccounts(accounts.filter(account => account.uuid !== element.uuid));
                }} className='remove-account' src={TrashIcon} alt="" />
            </div>)}
            <div className='account-items' id='add-account' onClick={() => {
                invoke('start_oauth').catch(e => {});
                toast.loading('Logging In.', {
                    id: 'currentLoginNotification',
                    position: 'bottom-center',
                    className: 'toast-notification',
                    duration: 60000,
                    iconTheme: {
                        primary: 'var(--icons-color-hover)',
                        secondary: 'var(--icons-color)'
                    }
                });
            }}>
                <div>
                    <img src={UserPlus} alt="" />
                    <span>Add Account</span>
                </div>
            </div>
        </div>
    );
}

export default AccountSelector;
