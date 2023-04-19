import { invoke } from '@tauri-apps/api/tauri';
import React, { memo } from 'react';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import '../styles/VersionMenu.css';

const releaseArray: string[] = await invoke('list_minecraft_versions', { versionType: 'release' });
const snapshotArray: string[] = await invoke('list_minecraft_versions', { versionType: 'snapshot' });
const oldBetaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_beta' });
const oldAlphaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_alpha' });

interface VersionMenuProps {
    selectedVersion: string
    setSelectedVersion: (version: string) => void
    selectedVersionType: string
    setSelectedVersionType: (type: string) => void
}

function VersionMenu(props: VersionMenuProps): JSX.Element {
    return (
        <div className='version-menu'>
            <div className='version-tabs'>
                <div className={`version-type ${props.selectedVersionType === 'release' ? 'selected' : ''}`} onClick={() => { props.setSelectedVersionType('release'); }}><span>Release</span></div>
                <div className={`version-type ${props.selectedVersionType === 'snapshot' ? 'selected' : ''}`} onClick={() => { props.setSelectedVersionType('snapshot'); }}><span>Snapshot</span></div>
                <div className={`version-type ${props.selectedVersionType === 'old_beta' ? 'selected' : ''}`} onClick={() => { props.setSelectedVersionType('old_beta'); }}><span>Beta</span></div>
                <div className={`version-type ${props.selectedVersionType === 'old_alpha' ? 'selected' : ''}`} onClick={() => { props.setSelectedVersionType('old_alpha'); }}><span>Alpha</span></div>
            </div>
            <img className="input-image" src={props.selectedVersion.length > 0 ? CheckIcon : AlertIcon} alt="" />
            <div className='version-container'>
                {props.selectedVersionType === 'release' && releaseArray.map((element, index) => <div key={index} className={`version ${props.selectedVersion === element ? 'selected' : ''}`} onClick={() => { props.setSelectedVersion(element); } }><span>{props.selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                {props.selectedVersionType === 'snapshot' && snapshotArray.map((element, index) => <div key={index} className={`version ${props.selectedVersion === element ? 'selected' : ''}`} onClick={() => { props.setSelectedVersion(element); } }><span>{props.selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                {props.selectedVersionType === 'old_beta' && oldBetaArray.map((element, index) => <div key={index} className={`version ${props.selectedVersion === element ? 'selected' : ''}`} onClick={() => { props.setSelectedVersion(element); } }><span>{props.selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                {props.selectedVersionType === 'old_alpha' && oldAlphaArray.map((element, index) => <div key={index} className={`version ${props.selectedVersion === element ? 'selected' : ''}`} onClick={() => { props.setSelectedVersion(element); } }><span>{props.selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
            </div>
        </div>
    );
}

export default memo(VersionMenu);
