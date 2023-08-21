import { type JSX } from 'solid-js';
import { Window } from '@tauri-apps/plugin-window';
import './TitleBar.scss';
import {
    AppIcon,
    ArrowIcon,
    MinusIcon,
    NotificationIcon,
    SquareIcon,
    XIcon,
} from '../../../assets/icons/Icons';
import { type PageNavigationProps } from '../../../data/models';
import { pages } from '../../../data/constants';
import { Pages } from '../../../data/enums';

function TitleBar(props: PageNavigationProps): JSX.Element {
    return (
        <div data-tauri-drag-region id='titlebar'>
            <div class='clickable hoverable'>
                <NotificationIcon />
            </div>
            <div
                class='clickable hoverable'
                onClick={() => {
                    Window.getCurrent().minimize().catch();
                }}
            >
                <MinusIcon />
            </div>
            <div
                class='clickable hoverable'
                onClick={() => {
                    Window.getCurrent().maximize().catch();
                }}
            >
                <SquareIcon />
            </div>
            <div
                class='clickable hoverable'
                onClick={() => {
                    Window.getCurrent().close().catch();
                }}
            >
                <XIcon />
            </div>
            <div id='navbar'>
                <span
                    onClick={() => {
                        props.setCurrentPage(Pages.Home);
                    }}
                >
                    <AppIcon />
                    ATLAS
                </span>
                <ArrowIcon />
                {pages[props.currentPage].name}
            </div>
        </div>
    );
}

export default TitleBar;
