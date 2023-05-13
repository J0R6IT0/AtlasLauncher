import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect } from 'react';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import '../styles/ForgeVersionMenu.css';

type ForgeVersionData = Record<string, string[]>;

let versions: ForgeVersionData[];
const keys: string[] = [];
await invoke('get_forge_versions').then((obj) => {
    versions = obj as ForgeVersionData[];
    versions.reverse();
    versions.forEach(version => {
        const key = Object.keys(version)[0];
        version[key].reverse();
        keys.push(key);
    });
});

interface ForgeVersionMenuProps {
    autoScroll: boolean
    selectedMcVersion: string
    setSelectedMcVersion: (mcVersion: string) => void
    selectedVersion: string
    setSelectedVersion: (version: string) => void
}
function ForgeVersionMenu(props: ForgeVersionMenuProps): JSX.Element {
    useEffect(() => {
        props.setSelectedMcVersion(keys[0]);
    }, []);

    return (
        <div className='version-menu'>
            <img className="input-image forge" src={props.selectedVersion.length > 0 ? CheckIcon : AlertIcon} alt="" />
            <div className='forge-version-menu-container'>
                <div className="forge-container minecraft-versions">
                    {keys.map((mcId, key) => (
                        <div key={key} className={`version clickable ${props.selectedMcVersion === mcId ? 'selected' : ''}`} onClick={() => {
                            props.setSelectedMcVersion(mcId);
                            if (props.selectedMcVersion !== mcId) {
                                props.setSelectedVersion('');
                            }
                        }}>
                            <span>{props.selectedMcVersion === mcId && <div className='dot'></div>}{mcId}</span>
                        </div>
                    ))}
                </div>
                <div className='forge-container forge-versions'>
                    {versions[keys.indexOf(props.selectedMcVersion)] !== undefined && versions[keys.indexOf(props.selectedMcVersion)][props.selectedMcVersion].map((element, key) => (
                        <div key={key} className={`version clickable ${props.selectedVersion === element ? 'selected' : ''}`} onClick={() => {
                            props.setSelectedVersion(element);
                        }}>
                            <span>{props.selectedVersion === element && <div className='dot'></div>}{element.split('-')[1] !== undefined ? element.split('-')[1] : element.split('-')[0] }</span>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}

export default ForgeVersionMenu;
