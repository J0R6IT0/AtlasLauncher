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

function VersionMenu(props: VersionMenuProps): JSX.Element {
    const [versions, setVersions] = useState<MinecraftVersion[]>([]);
    const [selectedVersionType, setSelectedVersionType] = useState('release');
    const selectedVersionRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        invoke('get_minecraft_versions')
            .then((obj) => {
                setVersions(obj as MinecraftVersion[]);
                if (selectedVersionRef.current !== null && props.autoScroll) {
                    selectedVersionRef.current.scrollIntoView();
                }
            })
            .catch((e) => {});
    }, []);

    return (
        <div className='version-menu'>
            <div className='version-tabs'>
                <div
                    className={`version-type clickable ${
                        selectedVersionType === 'release' ? 'selected' : ''
                    }`}
                    onClick={() => {
                        setSelectedVersionType('release');
                    }}
                >
                    <span>Release</span>
                </div>
                <div
                    className={`version-type clickable ${
                        selectedVersionType === 'snapshot' ? 'selected' : ''
                    }`}
                    onClick={() => {
                        setSelectedVersionType('snapshot');
                    }}
                >
                    <span>Snapshot</span>
                </div>
                <div
                    className={`version-type clickable ${
                        selectedVersionType === 'old_beta' ? 'selected' : ''
                    }`}
                    onClick={() => {
                        setSelectedVersionType('old_beta');
                    }}
                >
                    <span>Beta</span>
                </div>
                <div
                    className={`version-type clickable ${
                        selectedVersionType === 'old_alpha' ? 'selected' : ''
                    }`}
                    onClick={() => {
                        setSelectedVersionType('old_alpha');
                    }}
                >
                    <span>Alpha</span>
                </div>
            </div>
            {props.mcVersion.length > 0 ? <CheckIcon /> : <AlertTriangleIcon />}
            <div className='version-container'>
                {selectedVersionType === 'release' &&
                    versions
                        .filter((element) => element.type === 'release')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.mcVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.mcVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setMcVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.mcVersion === element.id && (
                                        <div className='dot'></div>
                                    )}
                                    {element.id}
                                </span>
                            </div>
                        ))}
                {selectedVersionType === 'snapshot' &&
                    versions
                        .filter((element) => element.type === 'snapshot')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.mcVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.mcVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setMcVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.mcVersion === element.id && (
                                        <div className='dot'></div>
                                    )}
                                    {element.id}
                                </span>
                            </div>
                        ))}
                {selectedVersionType === 'old_beta' &&
                    versions
                        .filter((element) => element.type === 'old_beta')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.mcVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.mcVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setMcVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.mcVersion === element.id && (
                                        <div className='dot'></div>
                                    )}
                                    {element.id}
                                </span>
                            </div>
                        ))}
                {selectedVersionType === 'old_alpha' &&
                    versions
                        .filter((element) => element.type === 'old_alpha')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.mcVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.mcVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setMcVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.mcVersion === element.id && (
                                        <div className='dot'></div>
                                    )}
                                    {element.id}
                                </span>
                            </div>
                        ))}
            </div>
        </div>
    );
}

export default memo(VersionMenu);
