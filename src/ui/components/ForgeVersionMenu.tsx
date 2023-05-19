import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useState } from 'react';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import '../styles/ForgeVersionMenu.css';

type ForgeVersionData = Record<string, string[]>;

interface ForgeVersionMenuProps {
    autoScroll: boolean
    selectedMcVersion: string
    setSelectedMcVersion: (mcVersion: string) => void
    selectedVersion: string
    setSelectedVersion: (version: string) => void
}

function ForgeVersionMenu(props: ForgeVersionMenuProps): JSX.Element {
    const [versions, setVersions] = useState<ForgeVersionData[]>([]);
    const [keys, setKeys] = useState<string[]>([]);

    useEffect(() => {
        let newVersions: ForgeVersionData[] = [];
        const keys: string[] = [];
        invoke('get_forge_versions').then((obj) => {
            console.log('invoke');
            newVersions = obj as ForgeVersionData[];
            newVersions.reverse();
            newVersions.forEach(version => {
                const key = Object.keys(version)[0];
                version[key].reverse();
                keys.push(key);
            });
            setVersions(newVersions);
            setKeys(keys);
            props.setSelectedMcVersion(keys[0]);
        }).catch(e => {});
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
