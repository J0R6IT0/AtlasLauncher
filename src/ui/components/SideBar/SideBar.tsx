import './SideBar.scss';
import { pages } from '../../../data/constants';
import { For, type JSX } from 'solid-js';
import { UserIcon } from '../../../assets/icons/Icons';
import { Pages } from '../../../data/enums';
import { type MinecraftAccount } from '../../../data/models';

interface SideBarProps {
    currentPage: Pages;
    setCurrentPage: (page: Pages) => void;
    setAccountsMenuVisible: (visible?: boolean) => void;
    activeAccount: MinecraftAccount | null;
}

function SideBar(props: SideBarProps): JSX.Element {
    return (
        <div id='sidebar'>
            <For each={pages}>
                {(page) => (
                    <div
                        class='clickable hoverable'
                        classList={{
                            selected: props.currentPage === page.page,
                        }}
                        onClick={() => {
                            props.setCurrentPage(page.page);
                        }}
                    >
                        <div />
                        <page.icon />
                    </div>
                )}
            </For>
            <div
                id='account'
                class='clickable hoverable'
                onClick={() => {
                    props.setAccountsMenuVisible(true);
                }}
            >
                {props.activeAccount !== null ? (
                    <img
                        src={`data:image/png;base64,${props.activeAccount.avatar_64px}`}
                    />
                ) : (
                    <UserIcon />
                )}
            </div>
        </div>
    );
}

export default SideBar;
