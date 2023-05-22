import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useState } from 'react';
import '../styles/FabricVersionMenu.css';
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
    useEffect(() => {
        invoke('get_fabric_minecraft_versions', { isQuilt: !!props.isQuilt })
            .then((obj) => {
                setMcVersions(obj as FabricMinecraftVersion[]);
                props.setMcVersion(
                    (obj as FabricMinecraftVersion[]).filter(
                        (mcVersion) => mcVersion.stable === stable
                    )[0].version
                );
            })
            .catch((e) => {});
        invoke('get_fabric_versions', { isQuilt: !!props.isQuilt })
            .then((obj) => {
                setLoaderVersions(obj as FabricVersion[]);
            })
            .catch((e) => {});
    }, []);

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
            </div>
            {props.modloaderVersion.length > 0 ? (
                <CheckIcon />
            ) : (
                <AlertTriangleIcon />
            )}
            <div className='forge-version-menu-container fabric'>
                <div className='forge-container minecraft-versions fabric'>
                    {mcVersions
                        .filter((mcVersion) => mcVersion.stable === stable)
                        .map((mcVersion, key) => (
                            <div
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
                            </div>
                        ))}
                </div>
                <div className='forge-container forge-versions'>
                    {loaderVersions.map((element, key) => (
                        <div
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
                                {props.mcVersion === element.version && (
                                    <div className='dot'></div>
                                )}
                                {element.version.split('+')[0]}
                            </span>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}

export default FabricVersionMenu;
