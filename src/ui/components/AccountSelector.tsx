import { invoke } from '@tauri-apps/api/tauri';
import React, { useRef } from 'react';
import toast from 'react-hot-toast';
import { type AccountInfo } from '../../App';

import '../styles/AccountSelector.css';
import mountAnimationHandler from '../../utils/mountAnimationHandler';
import { TrashIcon, UserPlusIcon } from '../../assets/icons/Icons';

interface AccountSelectorProps {
    onClose: () => void;
    updateAccounts: () => void;
    accounts: AccountInfo[];
}

function AccountSelector(props: AccountSelectorProps): JSX.Element {
    const accountSelectorRef = useRef<HTMLDivElement>(null);

    const handleClose = (): void => {
        accountSelectorRef.current?.classList.remove('visible');
        setTimeout(() => {
            props.onClose();
        }, 300);
    };

    mountAnimationHandler(accountSelectorRef, handleClose);

    return (
        <div className='account-selector' ref={accountSelectorRef}>
            {props.accounts.map((element, index) => (
                <div
                    key={index}
                    className={`account-items ${
                        element.active ? 'active' : ''
                    }`}
                >
                    <div
                        className='account clickable'
                        onClick={() => {
                            invoke('set_active_account', {
                                uuid: element.uuid,
                            }).catch((e) => {});
                            props.updateAccounts();
                        }}
                    >
                        <img
                            src={`data:image/png;base64,${element.avatar_64px}`}
                        />
                        <span>
                            {element.username}
                            <span id='active-account-label'>
                                {element.active ? '\nActive' : ''}
                            </span>
                        </span>
                    </div>
                    <div
                        onClick={() => {
                            invoke('remove_account', { uuid: element.uuid })
                                .then(() => {
                                    props.updateAccounts();
                                })
                                .catch((e) => {});
                        }}
                        className='remove-account clickable hover accent-text-secondary'
                    >
                        <TrashIcon />
                    </div>
                </div>
            ))}
            <div
                className='account-items clickable hover accent-text-secondary'
                id='add-account'
                onClick={() => {
                    invoke('start_oauth').catch((e) => {});
                    toast.loading('Logging In.', {
                        id: 'currentLoginNotification',
                    });
                }}
            >
                <div className='account'>
                    <UserPlusIcon />
                    <span>Add Account</span>
                </div>
            </div>
        </div>
    );
}

export default AccountSelector;
