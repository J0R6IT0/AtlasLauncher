import { invoke } from '@tauri-apps/api/tauri';
import React, { memo, useEffect, useRef, useState } from 'react';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import '../styles/VersionMenu.css';

interface MinecraftVersion {
    id: string;
    type: string;
}

interface VersionMenuProps {
    autoScroll: boolean;
    selectedVersion: string;
    setSelectedVersion: (version: string) => void;
    selectedVersionType: string;
    setSelectedVersionType: (type: string) => void;
}

function VersionMenu(props: VersionMenuProps): JSX.Element {
    const [versions, setVersions] = useState<MinecraftVersion[]>([]);
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
                        props.selectedVersionType === 'release'
                            ? 'selected'
                            : ''
                    }`}
                    onClick={() => {
                        props.setSelectedVersionType('release');
                    }}
                >
                    <span>Release</span>
                </div>
                <div
                    className={`version-type clickable ${
                        props.selectedVersionType === 'snapshot'
                            ? 'selected'
                            : ''
                    }`}
                    onClick={() => {
                        props.setSelectedVersionType('snapshot');
                    }}
                >
                    <span>Snapshot</span>
                </div>
                <div
                    className={`version-type clickable ${
                        props.selectedVersionType === 'old_beta'
                            ? 'selected'
                            : ''
                    }`}
                    onClick={() => {
                        props.setSelectedVersionType('old_beta');
                    }}
                >
                    <span>Beta</span>
                </div>
                <div
                    className={`version-type clickable ${
                        props.selectedVersionType === 'old_alpha'
                            ? 'selected'
                            : ''
                    }`}
                    onClick={() => {
                        props.setSelectedVersionType('old_alpha');
                    }}
                >
                    <span>Alpha</span>
                </div>
            </div>
            <img
                className='input-image'
                src={props.selectedVersion.length > 0 ? CheckIcon : AlertIcon}
                alt=''
            />
            <div className='version-container'>
                {props.selectedVersionType === 'release' &&
                    versions
                        .filter((element) => element.type === 'release')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.selectedVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.selectedVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setSelectedVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.selectedVersion === element.id && (
                                        <div className='dot'></div>
                                    )}
                                    {element.id}
                                </span>
                            </div>
                        ))}
                {props.selectedVersionType === 'snapshot' &&
                    versions
                        .filter((element) => element.type === 'snapshot')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.selectedVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.selectedVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setSelectedVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.selectedVersion === element.id && (
                                        <div className='dot'></div>
                                    )}
                                    {element.id}
                                </span>
                            </div>
                        ))}
                {props.selectedVersionType === 'old_beta' &&
                    versions
                        .filter((element) => element.type === 'old_beta')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.selectedVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.selectedVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setSelectedVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.selectedVersion === element.id && (
                                        <div className='dot'></div>
                                    )}
                                    {element.id}
                                </span>
                            </div>
                        ))}
                {props.selectedVersionType === 'old_alpha' &&
                    versions
                        .filter((element) => element.type === 'old_alpha')
                        .map((element, index) => (
                            <div
                                key={index}
                                ref={
                                    props.selectedVersion === element.id
                                        ? selectedVersionRef
                                        : null
                                }
                                className={`version clickable ${
                                    props.selectedVersion === element.id
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setSelectedVersion(element.id);
                                }}
                            >
                                <span>
                                    {props.selectedVersion === element.id && (
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
