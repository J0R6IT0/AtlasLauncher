import React, { useState } from 'react';
import SideBar from './ui/components/SideBar';
import NewInstance from './ui/pages/NewInstance';
import Library from './ui/pages/Library';
import AccountSelector from './ui/components/AccountSelector';
import './ui/styles/App.css';
import { appWindow } from '@tauri-apps/api/window';
import MinusIcon from './assets/icons/minus.svg';
import SquareIcon from './assets/icons/square.svg';
import XIcon from './assets/icons/x.svg';
import { Toaster } from 'react-hot-toast';
import BackgroundImage from './assets/images/minecraft-background.jpg';
import UserIcon from './assets/icons/user.svg';
import BellIcon from './assets/icons/bell.svg';

function App(): JSX.Element {
    const [activePage, setActivePage] = useState(1);
    const [accountSelectorActive, setAccountSelectorActive] = useState(false);

    function accountSelectorHandle(): void {
        setAccountSelectorActive(!accountSelectorActive);
    }

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
                {activePage === 2 && <Library />}
            </div>
            <div className='secondary-buttons'>
                <div>
                    <img src={BellIcon} />
                </div>
                <div onClick={accountSelectorHandle} id='accounts-button'>
                    <img src={UserIcon} />
                </div>
            </div>
            <Toaster />
        </div>
    );
}

export default App;
