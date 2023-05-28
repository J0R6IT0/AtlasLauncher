import { invoke } from '@tauri-apps/api/tauri';
import React, { memo, useEffect, useRef, useState } from 'react';
import '../styles/VersionMenu.css';
import { AlertTriangleIcon, CheckIcon } from '../../assets/icons/Icons';

interface MinecraftVersion {
    id: string;
    type: string;
}

interface VersionMenuProps {
    autoScroll: boolean;
    mcVersion: string;
    setMcVersion: (version: string) => void;
}

const versionTypes = [
    { id: 'release', pretty: 'Release' },
    { id: 'snapshot', pretty: 'Snapshot' },
    { id: 'old_beta', pretty: 'Beta' },
    { id: 'old_alpha', pretty: 'Alpha' },
];

let versionCache: MinecraftVersion[] = [];

function VersionMenu(props: VersionMenuProps): JSX.Element {
    const [versions, setVersions] = useState<MinecraftVersion[]>(versionCache);
    const [selectedVersionType, setSelectedVersionType] = useState('');
    const selectedVersionRef = useRef<HTMLLIElement>(null);

    function scroll(): void {
        if (props.autoScroll) {
            const thisVersion = versionCache.filter(
                (element) => element.id === props.mcVersion
            );
            if (thisVersion.length > 0) {
                setSelectedVersionType(thisVersion[0].type);
            }
        } else {
            setSelectedVersionType('release');
        }
    }

    useEffect(() => {
        if (versions.length <= 0) {
            invoke('get_minecraft_versions')
                .then((obj) => {
                    versionCache = obj as MinecraftVersion[];
                    setVersions(versionCache);
                    scroll();
                })
                .catch((e) => {});
        } else {
            scroll();
        }
    }, []);

    useEffect(() => {
        if (selectedVersionRef.current !== null) {
            selectedVersionRef.current.scrollIntoView();
        }
    }, [selectedVersionType]);

    return (
        <div className='version-menu'>
            <div className='version-tabs'>
                {selectedVersionType.length > 0 &&
                    versionTypes.map((element, key) => (
                        <div
                            key={key}
                            className={`version-type clickable ${
                                selectedVersionType === element.id
                                    ? 'selected'
                                    : ''
                            }`}
                            onClick={() => {
                                setSelectedVersionType(element.id);
                            }}
                        >
                            <span>{element.pretty}</span>
                        </div>
                    ))}
                {props.mcVersion.length > 0 ? (
                    <CheckIcon />
                ) : (
                    <AlertTriangleIcon />
                )}
            </div>
            <div className='version-container'>
                {versions
                    .filter((element) => element.type === selectedVersionType)
                    .map((element, index) => (
                        <li
                            key={index}
                            ref={
                                props.mcVersion === element.id
                                    ? selectedVersionRef
                                    : null
                            }
                            className={`version clickable ${
                                props.mcVersion === element.id ? 'selected' : ''
                            }`}
                            onClick={() => {
                                props.setMcVersion(element.id);
                            }}
                        >
                            <span>
                                {props.mcVersion === element.id && (
                                    <div className='dot' />
                                )}
                                {element.id}
                            </span>
                        </li>
                    ))}
            </div>
        </div>
    );
}

export default memo(VersionMenu);
