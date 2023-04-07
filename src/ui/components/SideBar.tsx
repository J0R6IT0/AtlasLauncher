import React from 'react';
import PlusIcon from '../../assets/icons/plus.svg';
import HomeIcon from '../../assets/icons/home.svg';
import GridIcon from '../../assets/icons/grid.svg';
import PackageIcon from '../../assets/icons/package.svg';
import SettingsIcon from '../../assets/icons/settings.svg';

import '../styles/SideBar.css';

interface SideBarProps {
    setActivePage: (page: number) => void
}

function SideBar({ setActivePage }: SideBarProps): JSX.Element {
    return (
        <div id='side-bar' className='side-bar'>
            <div className='side-bar-items'>
                <img src={HomeIcon}/>
            </div>
            <div className='side-bar-items' onClick={() => { setActivePage(2); }}>
                <img src={GridIcon}/>
            </div>
            <div className='side-bar-items new-icon' onClick={() => { setActivePage(1); }}>
                <img src={PlusIcon}/>
            </div>
            <div className='side-bar-items'>
                <img src={PackageIcon}/>
            </div>
            <div className='side-bar-items'>
                <img src={SettingsIcon}/>
            </div>
        </div>
    );
}

export default SideBar;
