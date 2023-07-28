import { invoke } from '@tauri-apps/api/tauri';
import { TrashIcon, UserAddIcon } from '../../../assets/icons/Icons';
import { type MinecraftAccount } from '../../../data/models';
import BaseMenu from '../BaseMenu/BaseMenu';
import './AccountsMenu.scss';
import { type JSX } from 'solid-js';

interface AccountMenuProps {
    accounts: MinecraftAccount[];
    visible: boolean;
    setVisible: (visible?: boolean) => void;
    refreshAccounts: () => void;
}

function AccountMenu(props: AccountMenuProps): JSX.Element {
    const handleAccountsRefresh = () => {
        props.refreshAccounts();
    };

    return (
        <BaseMenu
            visible={props.visible}
            setVisible={props.setVisible}
            items={props.accounts
                .map((account) => (
                    <div
                        class='account-item'
                        classList={{ active: account.active }}
                    >
                        <div
                            class='account-details clickable'
                            onClick={() => {
                                invoke('set_active_account', {
                                    uuid: account.uuid,
                                })
                                    .then(handleAccountsRefresh)
                                    .catch();
                            }}
                        >
                            <img
                                src={`data:image/png;base64,${account.avatar_64}`}
                            />
                            <span>
                                {account.username}
                                {account.active && (
                                    <p id='active-label'>ACTIVE</p>
                                )}
                            </span>
                        </div>
                        <div
                            class='remove-account clickable hoverable danger'
                            onClick={() => {
                                invoke('remove_account', {
                                    uuid: account.uuid,
                                })
                                    .then(handleAccountsRefresh)
                                    .catch();
                            }}
                        >
                            <TrashIcon />
                        </div>
                    </div>
                ))
                .concat(
                    <div
                        class='account-item hoverable clickable'
                        onClick={() => {
                            invoke('start_msauth').catch();
                        }}
                    >
                        <div
                            class='account-details clickable'
                            style={{ width: '100%' }}
                        >
                            <UserAddIcon />
                            <span style={{ 'margin-right': '0.5rem' }}>
                                Add Account
                            </span>
                        </div>
                    </div>
                )}
        />
    );
}

export default AccountMenu;
