import { invoke } from '@tauri-apps/api/tauri';
import React, { memo, useEffect, useRef, useState } from 'react';
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

let versionCache: ForgeVersionData[] = [];
let keyCache: string[] = [];

function ForgeVersionMenu(props: ForgeVersionMenuProps): JSX.Element {
    const [versions, setVersions] = useState<ForgeVersionData[]>(versionCache);
    const [keys, setKeys] = useState<string[]>(keyCache);

    const selectedMcVersionRef = useRef<HTMLLIElement>(null);
    const selectedVersionRef = useRef<HTMLLIElement>(null);

    function scroll(): void {
        if (props.autoScroll) {
            if (selectedMcVersionRef.current !== null && props.autoScroll) {
                selectedMcVersionRef.current.scrollIntoView();
            }
            if (selectedVersionRef.current !== null && props.autoScroll) {
                selectedVersionRef.current.scrollIntoView();
            }
        }
    }

    useEffect(() => {
        if (versionCache.length <= 0 || keyCache.length <= 0) {
            invoke('get_forge_versions')
                .then((obj) => {
                    const newVersions = obj as ForgeVersionData[];
                    const newKeys: string[] = [];
                    newVersions.reverse();
                    newVersions.forEach((version) => {
                        const key = Object.keys(version)[0];
                        version[key].reverse();
                        newKeys.push(key);
                    });
                    versionCache = newVersions;
                    keyCache = newKeys;
                    setVersions(versionCache);
                    setKeys(newKeys);
                    scroll();
                })
                .catch((e) => {});
        } else {
            scroll();
        }
    }, []);

    useEffect(() => {
        if (!props.autoScroll) {
            props.setMcVersion(keys[0]);
        }
    }, [keys]);

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

export default memo(ForgeVersionMenu);
