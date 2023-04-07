import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useState } from 'react';
import UserPlus from '../../assets/icons/user-plus.svg';
import { once } from '@tauri-apps/api/event';
import toast from 'react-hot-toast';

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

const accountsFirstRun: AccountInfo[] = await invoke('get_accounts').catch(e => {}) as AccountInfo[];
const activeAccountFirstRun = await invoke('get_active_account').catch(e => {}) as string;

function AccountSelector(): JSX.Element {
    const [accounts, setAccounts] = useState(accountsFirstRun);
    const [activeAccount, setActiveAccount] = useState(activeAccountFirstRun);

    async function getAccounts(): Promise<void> {
        const accounts = await invoke('get_accounts').catch(e => {}) as AccountInfo[];
        const activeAccount = await invoke('get_active_account').catch(e => {}) as string;
        setAccounts(accounts);
        setActiveAccount(activeAccount);
    }

    useEffect(() => {
        const element = document.getElementById('account-selector');
        const button = document.getElementById('accounts-button');
        const menuToggle = document.getElementById('menu-toggle');

        once('auth', (event: LoginEvent) => {
            getAccounts().catch(e => {});
            console.log('e');

            if (event.payload.status === 'Success') {
                toast.success(event.payload.message, {
                    id: 'currentLoginNotification',
                    duration: 6000,
                    position: 'bottom-center',
                    iconTheme: {
                        primary: '#212128',
                        secondary: '#e44b7f'
                    }
                });
            } else if (event.payload.status === 'Error') {
                toast.error(event.payload.message, {
                    id: 'currentLoginNotification',
                    duration: 10000,
                    position: 'bottom-center',
                    iconTheme: {
                        primary: '#212128',
                        secondary: '#e44b7f'
                    }
                });
            } else if (event.payload.status === 'Loading') {
                toast.loading(event.payload.message, {
                    id: 'currentLoginNotification',
                    position: 'bottom-center',
                    className: 'toast-notification'
                });
            } else {
                toast.dismiss('currentLoginNotification');
            }
        }).catch(e => {});

        function clickHandler(event: Event): void {
            if (element !== null && button !== null && menuToggle !== null) {
                if (!element.contains(event.target as Node) && !button.contains(event.target as Node) && !menuToggle.contains(event.target as Node) && element.classList.contains('open')) {
                    element.classList.remove('open');
                    button?.classList.remove('open');
                }
            }
        }

        document.addEventListener('click', clickHandler);

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
        return () => {
            document.removeEventListener('click', clickHandler);
        };
    });

    return (
        <div className='account-selector' id='account-selector'>
            {accounts.map((element, index) => <div key={index} onClick={() => {
                invoke('set_active_account', { uuid: element.uuid }).catch(e => {});
                setActiveAccount(element.uuid);
            }} className='account-items'><img src={`https://crafatar.com/avatars/${element.uuid}?overlay`} alt="" /><span>{element.username}<span id='active-account-label'>{activeAccount === element.uuid ? '\nActive' : ''}</span></span></div>)}

            <div className='account-items' id='add-account' onClick={() => {
                invoke('start_oauth').catch(e => {});
                toast.loading('Logging In.', {
                    id: 'currentLoginNotification',
                    position: 'bottom-center',
                    className: 'toast-notification',
                    duration: 60000,
                    iconTheme: {
                        primary: '#e44b7f',
                        secondary: '#212128'
                    }
                });
            }}>
                <img src={UserPlus} alt="" />
                <span>Add Account</span>
            </div>
        </div>
    );
}

export default AccountSelector;
