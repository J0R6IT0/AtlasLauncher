import React from 'react';
import PlusIcon from '../../assets/icons/plus.svg';
import HomeIcon from '../../assets/icons/home.svg';
import GridIcon from '../../assets/icons/grid.svg';
import PackageIcon from '../../assets/icons/package.svg';

import '../styles/SideBar.css';

function SideBar(): JSX.Element {
    return (
        <div id='side-bar' className='side-bar'>
            <div className='side-bar-items'>
                <img src={HomeIcon}/>
            </div>
            <div className='side-bar-items'>
                <img src={GridIcon}/>
            </div>
            <div className='side-bar-items'>
                <img src={PlusIcon}/>
            </div>
            <div className='side-bar-items'>
                <img src={PackageIcon}/>
            </div>
        </div>
    );
}

export default SideBar;
