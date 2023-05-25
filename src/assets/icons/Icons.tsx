import React from 'react';
import './Icons.css';

const MinusIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <line x1='5' y1='12' x2='19' y2='12'></line>
    </svg>
);

const SquareIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <rect x='4' y='4' width='15' height='15' rx='2' ry='2'></rect>
    </svg>
);

const XIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <line x1='18' y1='6' x2='6' y2='18'></line>
        <line x1='6' y1='6' x2='18' y2='18'></line>
    </svg>
);

const HomeIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
        style={{ marginBottom: '0.15rem' }}
    >
        <path d='M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z'></path>
        <polyline points='9 22 9 12 15 12 15 22'></polyline>
    </svg>
);

const GridIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <rect x='3' y='3' width='7' height='7'></rect>
        <rect x='14' y='3' width='7' height='7'></rect>
        <rect x='14' y='14' width='7' height='7'></rect>
        <rect x='3' y='14' width='7' height='7'></rect>
    </svg>
);

const PlusIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <line x1='12' y1='5' x2='12' y2='19'></line>
        <line x1='5' y1='12' x2='19' y2='12'></line>
    </svg>
);

const PackageIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <line x1='16.5' y1='9.4' x2='7.5' y2='4.21'></line>
        <path d='M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z'></path>
        <polyline points='3.27 6.96 12 12.01 20.73 6.96'></polyline>
        <line x1='12' y1='22.08' x2='12' y2='12'></line>
    </svg>
);

const SettingsIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
        style={{ marginTop: '0.05rem' }}
    >
        <circle cx='12' cy='12' r='3'></circle>
        <path d='M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z'></path>
    </svg>
);

const BellIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9'></path>
        <path d='M13.73 21a2 2 0 0 1-3.46 0'></path>
    </svg>
);

const DownloadIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4'></path>
        <polyline points='7 10 12 15 17 10'></polyline>
        <line x1='12' y1='15' x2='12' y2='3'></line>
    </svg>
);

const UserIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2'></path>
        <circle cx='12' cy='7' r='4'></circle>
    </svg>
);

const UserPlusIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2'></path>
        <circle cx='8.5' cy='7' r='4'></circle>
        <line x1='20' y1='8' x2='20' y2='14'></line>
        <line x1='23' y1='11' x2='17' y2='11'></line>
    </svg>
);

const TrashIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <polyline points='3 6 5 6 21 6'></polyline>
        <path d='M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2'></path>
    </svg>
);

const AlertTriangleIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z'></path>
        <line x1='12' y1='9' x2='12' y2='13'></line>
        <line x1='12' y1='17' x2='12.01' y2='17'></line>
    </svg>
);

const CheckIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <polyline points='20 6 9 17 4 12'></polyline>
    </svg>
);

const BoxIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z'></path>
        <polyline points='3.27 6.96 12 12.01 20.73 6.96'></polyline>
        <line x1='12' y1='22.08' x2='12' y2='12'></line>
    </svg>
);

const ForgeIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        viewBox='0 0 58.37 38.27'
        className='custom'
    >
        <path d='m54.46.75H19.87c-.58,0-1.05.47-1.05,1.05v3.51H1.8c-.58,0-1.05.47-1.05,1.05,0,7.06,5.74,12.8,12.8,12.8,2.91,0,5.28,2.37,5.28,5.27s-2.37,5.28-5.28,5.28h-1c-.58,0-1.05.47-1.05,1.05v5.7c0,.58.47,1.05,1.05,1.05h9.21c.58,0,1.05-.47,1.05-1.05,0-2.52,3.7-4.65,8.07-4.65s8.07,2.13,8.07,4.65c0,.58.47,1.05,1.05,1.05h9.21c.58,0,1.05-.47,1.05-1.05v-5.7c0-.58-.47-1.05-1.05-1.05h-1c-2.91,0-5.28-2.37-5.28-5.28,0-4.94,4.02-8.97,8.97-8.97h2.56c1.74,0,3.16-1.42,3.16-3.16V3.91c0-1.74-1.42-3.16-3.16-3.16ZM13.55,17.06c-5.54,0-10.11-4.23-10.64-9.64h15.91v11.86c-1.34-1.37-3.21-2.23-5.28-2.23Zm41.97-4.74c0,.58-.47,1.05-1.05,1.05h-2.56c-6.11,0-11.07,4.97-11.07,11.07,0,4.05,3.28,7.35,7.33,7.38v3.6h-7.22c-.74-3.27-4.89-5.7-10.06-5.7s-9.32,2.43-10.06,5.7h-7.22v-3.6c4.05-.03,7.33-3.33,7.33-7.38V6.37h0v-3.51h33.54c.58,0,1.05.47,1.05,1.05v8.4Z' />
    </svg>
);

const FabricIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        viewBox='0 0 49.95 49.95'
        className='custom'
        style={{
            transform: 'scale(0.9) scaleX(-1)',
        }}
    >
        <path d='m46.04,8.88h-27.19c-.02-4.48-3.67-8.13-8.16-8.13C5.21.75.75,5.21.75,10.69v27.46c0,.16.04.31.1.45.7,5.46,5.04,9.79,10.5,10.5.14.06.29.1.45.1h34.24c1.74,0,3.16-1.42,3.16-3.16V12.04c0-1.74-1.42-3.16-3.16-3.16ZM2.86,10.69c0-4.32,3.51-7.83,7.83-7.83,3.34,0,6.05,2.72,6.05,6.05v20.9c-1.49-1.66-3.65-2.71-6.05-2.71-3.18,0-6.01,1.5-7.83,3.83V10.69Zm44.23,35.35c0,.58-.47,1.05-1.05,1.05H12.91c-5.54,0-10.05-4.51-10.05-10.05,0-4.32,3.51-7.83,7.83-7.83,3.34,0,6.05,2.72,6.05,6.05,0,2.55-2.08,4.63-4.63,4.63-1.93,0-3.49-1.57-3.49-3.5,0-1.42,1.16-2.58,2.59-2.58.58,0,1.05-.47,1.05-1.05s-.47-1.05-1.05-1.05c-2.59,0-4.69,2.1-4.69,4.69,0,3.09,2.51,5.6,5.6,5.6,3.72,0,6.74-3.02,6.74-6.74V10.98h27.19c.58,0,1.05.47,1.05,1.05v34Z' />
    </svg>
);

// Credits to modrinth/art https://github.com/modrinth/art/blob/main/Icons/LICENSE.txt
const QuiltIcon = (): JSX.Element => (
    <svg
        className='feather'
        xmlns='http://www.w3.org/2000/svg'
        viewBox='0 0 24 24'
    >
        <path d='M10.324 3.958a.354.354 0 0 0-.354-.354H3.955a.353.353 0 0 0-.354.354v5.999c0 .196.158.355.354.355H9.97a.355.355 0 0 0 .354-.355V3.958Zm0 10.087a.354.354 0 0 0-.354-.354H3.955a.353.353 0 0 0-.354.354v5.999c0 .196.158.355.354.355H9.97a.355.355 0 0 0 .354-.355v-5.999ZM20.408 3.958a.353.353 0 0 0-.354-.354h-6.015a.354.354 0 0 0-.354.354v5.999c0 .196.159.355.354.355h6.015a.354.354 0 0 0 .354-.355V3.958Zm1.705 14.239a.354.354 0 0 0 0-.5l-3.925-3.925a.355.355 0 0 0-.501 0l-3.915 3.915a.355.355 0 0 0 0 .501l3.925 3.924a.353.353 0 0 0 .501 0l3.915-3.915Z' />
    </svg>
);

const ToolIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z'></path>
    </svg>
);

const FolderIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z'></path>
    </svg>
);

const GlobeIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <circle cx='12' cy='12' r='10'></circle>
        <line x1='2' y1='12' x2='22' y2='12'></line>
        <path d='M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z'></path>{' '}
    </svg>
);

const CoffeeIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M18 8h1a4 4 0 0 1 0 8h-1'></path>
        <path d='M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z'></path>
        <line x1='6' y1='1' x2='6' y2='4'></line>
        <line x1='10' y1='1' x2='10' y2='4'></line>
        <line x1='14' y1='1' x2='14' y2='4'></line>
    </svg>
);

const PenToolIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <path d='M12 19l7-7 3 3-7 7-3-3z'></path>
        <path d='M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z'></path>
        <path d='M2 2l7.586 7.586'></path>
        <circle cx='11' cy='11' r='2'></circle>
    </svg>
);

const ArrowRightIcon = (): JSX.Element => (
    <svg
        xmlns='http://www.w3.org/2000/svg'
        className='feather'
        viewBox='0 0 24 24'
    >
        <line x1='5' y1='12' x2='19' y2='12'></line>
        <polyline points='12 5 19 12 12 19'></polyline>
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
    AlertTriangleIcon,
    CheckIcon,
    BoxIcon,
    ForgeIcon,
    FabricIcon,
    QuiltIcon,
    ToolIcon,
    FolderIcon,
    GlobeIcon,
    CoffeeIcon,
    PenToolIcon,
    ArrowRightIcon
};
