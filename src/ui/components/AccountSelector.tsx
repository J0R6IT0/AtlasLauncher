import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useRef } from 'react';
import UserPlus from '../../assets/icons/user-plus.svg';
import TrashIcon from '../../assets/icons/trash.svg';
import toast from 'react-hot-toast';
import { type AccountInfo } from '../../App';

import '../styles/AccountSelector.css';

interface AccountSelectorProps {
    onClose: () => void
    updateAccounts: () => void
    accounts: AccountInfo[]
    activeAccount: string
}

function AccountSelector(props: AccountSelectorProps): JSX.Element {
    const menuRef = useRef<HTMLDivElement>(null);

    const handleOutsideClick = (event: MouseEvent): void => {
        const menu = document.querySelector('.account-selector') as HTMLElement;
        if (!menu.contains(event.target as Node)) {
            menuRef.current?.classList.remove('visible');
            setTimeout(() => {
                props.onClose();
            }, 300);
        }
    };

    useEffect(() => {
        setTimeout(() => {
            menuRef.current?.classList.add('visible');
            document.addEventListener('click', handleOutsideClick);
        }, 10);
        return () => {
            document.removeEventListener('click', handleOutsideClick);
        };
    }, []);

    return (
        <div className='account-selector' ref={menuRef}>
            {props.accounts.map((element, index) => <div key={index} className={`account-items ${props.activeAccount === element.uuid ? 'active' : ''}`}>
                <div
                    onClick={() => {
                        invoke('set_active_account', { uuid: element.uuid }).catch(e => {});
                    }}>
                    <img src={`https://crafatar.com/avatars/${element.uuid}?overlay`} alt="" />
                    <span>{element.username}
                        <span id='active-account-label'>{props.activeAccount === element.uuid ? '\nActive' : ''}</span>
                    </span>
                </div>
                <img onClick={() => {
                    invoke('remove_account', { uuid: element.uuid }).then(() => {
                        props.updateAccounts();
                    }).catch(e => {});
                }} className='remove-account' src={TrashIcon} alt="" />
            </div>)}
            <div className='account-items' id='add-account' onClick={() => {
                invoke('start_oauth').catch(e => {});
                toast.loading('Logging In.', { id: 'currentLoginNotification' });
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
