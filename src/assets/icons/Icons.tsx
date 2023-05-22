import React from 'react';
import './Icons.css';

const MinusIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <line x1='5' y1='12' x2='19' y2='12'></line>
    </svg>
);

const SquareIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <rect x='4' y='4' width='15' height='15' rx='2' ry='2'></rect>
    </svg>
);

const XIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <line x1='18' y1='6' x2='6' y2='18'></line>
        <line x1='6' y1='6' x2='18' y2='18'></line>
    </svg>
);

const HomeIcon = (): JSX.Element => (
    <svg
        className='feather'
        viewBox='0 0 24 24'
        style={{ marginBottom: '0.15rem' }}
    >
        <path d='M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z'></path>
        <polyline points='9 22 9 12 15 12 15 22'></polyline>
    </svg>
);

const GridIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <rect x='3' y='3' width='7' height='7'></rect>
        <rect x='14' y='3' width='7' height='7'></rect>
        <rect x='14' y='14' width='7' height='7'></rect>
        <rect x='3' y='14' width='7' height='7'></rect>
    </svg>
);

const PlusIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <line x1='12' y1='5' x2='12' y2='19'></line>
        <line x1='5' y1='12' x2='19' y2='12'></line>
    </svg>
);

const PackageIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <line x1='16.5' y1='9.4' x2='7.5' y2='4.21'></line>
        <path d='M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z'></path>
        <polyline points='3.27 6.96 12 12.01 20.73 6.96'></polyline>
        <line x1='12' y1='22.08' x2='12' y2='12'></line>
    </svg>
);

const SettingsIcon = (): JSX.Element => (
    <svg
        className='feather'
        viewBox='0 0 24 24'
        style={{ marginTop: '0.05rem' }}
    >
        <circle cx='12' cy='12' r='3'></circle>
        <path d='M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z'></path>
    </svg>
);

const BellIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <path d='M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9'></path>
        <path d='M13.73 21a2 2 0 0 1-3.46 0'></path>
    </svg>
);

const DownloadIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <path d='M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4'></path>
        <polyline points='7 10 12 15 17 10'></polyline>
        <line x1='12' y1='15' x2='12' y2='3'></line>
    </svg>
);

const UserIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <path d='M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2'></path>
        <circle cx='12' cy='7' r='4'></circle>
    </svg>
);

const UserPlusIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <path d='M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2'></path>
        <circle cx='8.5' cy='7' r='4'></circle>
        <line x1='20' y1='8' x2='20' y2='14'></line>
        <line x1='23' y1='11' x2='17' y2='11'></line>
    </svg>
);

const TrashIcon = (): JSX.Element => (
    <svg className='feather' viewBox='0 0 24 24'>
        <polyline points='3 6 5 6 21 6'></polyline>
        <path d='M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2'></path>
    </svg>
);

export {
    MinusIcon,
    SquareIcon,
    XIcon,
    HomeIcon,
    GridIcon,
    PlusIcon,
    PackageIcon,
    SettingsIcon,
    BellIcon,
    DownloadIcon,
    UserIcon,
    UserPlusIcon,
    TrashIcon,
};
