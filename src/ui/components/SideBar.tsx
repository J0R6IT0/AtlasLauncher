import React from 'react';
import PlusIcon from '../../assets/icons/plus.svg';
import HomeIcon from '../../assets/icons/home.svg';
import GridIcon from '../../assets/icons/grid.svg';
import PackageIcon from '../../assets/icons/package.svg';
import SettingsIcon from '../../assets/icons/settings.svg';

import '../styles/SideBar.css';

interface SideBarProps {
    activePage: number
    setActivePage: (page: number) => void
}

function SideBar({ setActivePage, activePage }: SideBarProps): JSX.Element {
    return (
        <div id='side-bar' className='side-bar'>
            <div className='side-bar-items clickable'>
                <img src={HomeIcon}/>
            </div>
            <div className={`side-bar-items clickable ${activePage === 2 ? 'selected' : ''}`} onClick={() => { setActivePage(2); }}>
                <div className='side-bar-selector' />
                <img src={GridIcon}/>
            </div>
            <div className={`side-bar-items clickable ${activePage === 1 ? 'selected' : ''}`} onClick={() => { setActivePage(1); }}>
                <div className='side-bar-selector' />
                <img src={PlusIcon}/>
            </div>
            <div className='side-bar-items clickable'>
                <img src={PackageIcon}/>
            </div>
            <div className='side-bar-items clickable'>
                <img src={SettingsIcon}/>
            </div>
        </div>
    );
}

export default SideBar;
