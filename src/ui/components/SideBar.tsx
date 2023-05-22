import React from 'react';
import '../styles/SideBar.css';
import { type Pages, pages } from '../../App';

interface SideBarProps {
    activePage: Pages;
    setActivePage: (page: number) => void;
}

function SideBar({ setActivePage, activePage }: SideBarProps): JSX.Element {
    return (
        <div id='side-bar' className='side-bar'>
            {pages.map((element, key) => (
                <div
                    key={key}
                    className={`side-bar-items clickable hover ${
                        activePage === element.page
                            ? 'selected'
                            : 'accent-icons'
                    }`}
                    onClick={() => {
                        setActivePage(element.page);
                    }}
                >
                    <div className='side-bar-selector' />
                    <element.icon />
                </div>
            ))}
        </div>
    );
}

export default SideBar;
