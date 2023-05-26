import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useRef, useState } from 'react';
import '../styles/ForgeVersionMenu.css';
import { AlertTriangleIcon, CheckIcon } from '../../assets/icons/Icons';

type ForgeVersionData = Record<string, string[]>;

interface ForgeVersionMenuProps {
    autoScroll: boolean;
    mcVersion: string;
    setMcVersion: (mcVersion: string) => void;
    modloaderVersion: string;
    setModloaderVersion: (version: string) => void;
}

function ForgeVersionMenu(props: ForgeVersionMenuProps): JSX.Element {
    const [versions, setVersions] = useState<ForgeVersionData[]>([]);
    const [keys, setKeys] = useState<string[]>([]);

    const selectedMcVersionRef = useRef<HTMLLIElement>(null);
    const selectedVersionRef = useRef<HTMLLIElement>(null);

    useEffect(() => {
        let newVersions: ForgeVersionData[] = [];
        const keys: string[] = [];
        invoke('get_forge_versions')
            .then((obj) => {
                newVersions = obj as ForgeVersionData[];
                newVersions.reverse();
                newVersions.forEach((version) => {
                    const key = Object.keys(version)[0];
                    version[key].reverse();
                    keys.push(key);
                });
                setVersions(newVersions);
                setKeys(keys);
                if (props.mcVersion.length <= 0) {
                    props.setMcVersion(keys[0]);
                }
                if (selectedMcVersionRef.current !== null && props.autoScroll) {
                    selectedMcVersionRef.current.scrollIntoView();
                }
                if (selectedVersionRef.current !== null && props.autoScroll) {
                    selectedVersionRef.current.scrollIntoView();
                }
            })
            .catch((e) => {});
    }, []);

    return (
        <div className='version-menu'>
            <div className='version-container forge'>
                <div className='forge-container' style={{ width: '90%' }}>
                    {keys.map((mcId, key) => (
                        <li
                            ref={
                                props.mcVersion === mcId
                                    ? selectedMcVersionRef
                                    : null
                            }
                            key={key}
                            className={`version clickable ${
                                props.mcVersion === mcId ? 'selected' : ''
                            }`}
                            onClick={() => {
                                props.setMcVersion(mcId);
                                if (props.mcVersion !== mcId) {
                                    props.setModloaderVersion('');
                                }
                            }}
                        >
                            <span>
                                {props.mcVersion === mcId && (
                                    <div className='dot'></div>
                                )}
                                {mcId}
                            </span>
                        </li>
                    ))}
                </div>
                <div className='forge-container'>
                    {versions[keys.indexOf(props.mcVersion)] !== undefined &&
                        versions[keys.indexOf(props.mcVersion)][
                            props.mcVersion
                        ].map((element, key) => (
                            <li
                                ref={
                                    props.modloaderVersion === element
                                        ? selectedVersionRef
                                        : null
                                }
                                key={key}
                                className={`version clickable ${
                                    props.modloaderVersion === element
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setModloaderVersion(element);
                                }}
                            >
                                <span>
                                    {props.modloaderVersion === element && (
                                        <div className='dot'></div>
                                    )}
                                    {element.split('-')[1] !== undefined
                                        ? element.split('-')[1]
                                        : element.split('-')[0]}
                                </span>
                            </li>
                        ))}
                </div>
            </div>
            {props.modloaderVersion.length > 0 ? (
                <CheckIcon />
            ) : (
                <AlertTriangleIcon />
            )}
        </div>
    );
}

export default ForgeVersionMenu;
