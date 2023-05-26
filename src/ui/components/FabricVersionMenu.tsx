import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useRef, useState } from 'react';
import { AlertTriangleIcon, CheckIcon } from '../../assets/icons/Icons';

interface FabricMinecraftVersion {
    version: string;
    stable: boolean;
}

interface FabricVersion {
    version: string;
}

interface ForgeVersionMenuProps {
    autoScroll: boolean;
    mcVersion: string;
    setMcVersion: (mcVersion: string) => void;
    modloaderVersion: string;
    setModloaderVersion: (version: string) => void;
    isQuilt: boolean;
}
function FabricVersionMenu(props: ForgeVersionMenuProps): JSX.Element {
    const [stable, setStable] = useState(true);
    const [mcVersions, setMcVersions] = useState<FabricMinecraftVersion[]>([]);
    const [loaderVersions, setLoaderVersions] = useState<FabricVersion[]>([]);

    const selectedMcVersionRef = useRef<HTMLLIElement>(null);
    const selectedVersionRef = useRef<HTMLLIElement>(null);

    useEffect(() => {
        invoke('get_fabric_minecraft_versions', { isQuilt: !!props.isQuilt })
            .then((obj) => {
                setMcVersions(obj as FabricMinecraftVersion[]);
                if (props.mcVersion.length <= 0) {
                    props.setMcVersion(
                        (obj as FabricMinecraftVersion[]).filter(
                            (mcVers) => mcVers.stable === stable
                        )[0].version
                    );
                } else {
                    setStable(
                        (obj as FabricMinecraftVersion[]).filter(
                            (mcVers) => mcVers.version === props.mcVersion
                        )[0].stable
                    );
                }
            })
            .catch((e) => {});
        invoke('get_fabric_versions', { isQuilt: !!props.isQuilt })
            .then((obj) => {
                setLoaderVersions(obj as FabricVersion[]);
            })
            .catch((e) => {});
    }, []);

    useEffect(() => {
        if (props.autoScroll) {
            if (selectedMcVersionRef.current !== null) {
                selectedMcVersionRef.current.scrollIntoView();
            }
            if (selectedVersionRef.current !== null) {
                selectedVersionRef.current.scrollIntoView();
            }
        }
    }, [mcVersions, loaderVersions]);

    return (
        <div className='version-menu'>
            <div className='version-tabs'>
                <div
                    className={`version-type clickable ${
                        stable ? 'selected' : ''
                    }`}
                    onClick={() => {
                        setStable(true);
                    }}
                >
                    <span>Release</span>
                </div>
                <div
                    className={`version-type clickable ${
                        !stable ? 'selected' : ''
                    }`}
                    onClick={() => {
                        setStable(false);
                    }}
                >
                    <span>Snapshot</span>
                </div>
                {props.modloaderVersion.length > 0 ? (
                    <CheckIcon />
                ) : (
                    <AlertTriangleIcon />
                )}
            </div>
            <div className='version-container forge'>
                <div className='forge-container'>
                    {mcVersions
                        .filter((mcVersion) => mcVersion.stable === stable)
                        .map((mcVersion, key) => (
                            <li
                                ref={
                                    props.mcVersion === mcVersion.version
                                        ? selectedMcVersionRef
                                        : null
                                }
                                key={key}
                                className={`version clickable ${
                                    props.mcVersion === mcVersion.version
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setMcVersion(mcVersion.version);
                                    if (props.mcVersion !== mcVersion.version) {
                                        props.setModloaderVersion('');
                                    }
                                }}
                            >
                                <span>
                                    {props.mcVersion === mcVersion.version && (
                                        <div className='dot'></div>
                                    )}
                                    {mcVersion.version}
                                </span>
                            </li>
                        ))}
                </div>
                <div className='forge-container forge-versions'>
                    {loaderVersions.map((element, key) => (
                        <li
                            ref={
                                props.modloaderVersion === element.version
                                    ? selectedVersionRef
                                    : null
                            }
                            key={key}
                            className={`version clickable ${
                                props.modloaderVersion === element.version
                                    ? 'selected'
                                    : ''
                            }`}
                            onClick={() => {
                                props.setModloaderVersion(element.version);
                            }}
                        >
                            <span>
                                {props.modloaderVersion === element.version && (
                                    <div className='dot'></div>
                                )}
                                {element.version.split('+')[0]}
                            </span>
                        </li>
                    ))}
                </div>
            </div>
        </div>
    );
}

export default FabricVersionMenu;
