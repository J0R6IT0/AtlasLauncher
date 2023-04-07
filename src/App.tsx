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

function App(): JSX.Element {
    const [activePage, setActiveButton] = useState(1);

    /*

    */

    return (
        <div className='container'>
            <img className='background-image' src={BackgroundImage} alt="" />
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
                    <img src={XIcon} alt="close" />
                </div>
            </div>
            <SideBar />
            <AccountSelector />
            <div className='content'>
                {activePage === 1 && <NewInstance />}
                {activePage === 2 && <Library />}
            </div>
            <Toaster />
        </div>
    );
}

export default App;
