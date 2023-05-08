import { invoke } from '@tauri-apps/api/tauri';
import React from 'react';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import '../styles/ForgeVersionMenu.css';

interface ForgeVersionsData {
    mc_id: string
    versions: ForgeVersionData[]
}

interface ForgeVersionData {
    id: string
    url?: string
    installer?: string
    sha1?: string
    size?: string
}

let versions: ForgeVersionsData[];
const keys: string[] = [];
await invoke('get_forge_versions').then((obj) => {
    versions = obj as ForgeVersionsData[];
    versions.forEach(version => {
        version.versions.reverse();
        keys.push(version.mc_id);
    });
});

keys.reverse();

interface ForgeVersionMenuProps {
    autoScroll: boolean
    selectedMcVersion: string
    setSelectedMcVersion: (mcVersion: string) => void
    selectedVersion: string
    setSelectedVersion: (version: string) => void
}
function ForgeVersionMenu(props: ForgeVersionMenuProps): JSX.Element {
    return (
        <div className='version-menu'>
            <img className="input-image forge" src={props.selectedVersion.length > 0 ? CheckIcon : AlertIcon} alt="" />
            <div className='forge-version-menu-container'>
                <div className="forge-container minecraft-versions">
                    {keys.map((mcId, key) => (
                        <div key={key} className={`version clickable ${props.selectedMcVersion === mcId ? 'selected' : ''}`} onClick={() => {
                            props.setSelectedMcVersion(mcId);
                        }}>
                            <span>{props.selectedMcVersion === mcId && <div className='dot'></div>}{mcId}</span>
                        </div>
                    ))}
                </div>
                <div className='forge-container forge-versions'>
                    {versions.filter(version => version.mc_id === props.selectedMcVersion)[0].versions.map((element, key) => (
                        <div key={key} className={`version clickable ${props.selectedVersion === element.id ? 'selected' : ''}`} onClick={() => {
                            props.setSelectedVersion(element.id);
                        }}>
                            <span>{props.selectedVersion === element.id && <div className='dot'></div>}{element.id}</span>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}

export default ForgeVersionMenu;
