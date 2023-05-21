import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useState } from 'react';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import '../styles/FabricVersionMenu.css';

interface FabricMinecraftVersion {
    version: string;
    stable: boolean;
}

interface FabricVersion {
    version: string;
}

interface ForgeVersionMenuProps {
    autoScroll: boolean;
    selectedMcVersion: string;
    setSelectedMcVersion: (mcVersion: string) => void;
    selectedVersion: string;
    setSelectedVersion: (version: string) => void;
    quilt: boolean;
}
function FabricVersionMenu(props: ForgeVersionMenuProps): JSX.Element {
    const [stable, setStable] = useState(true);
    const [mcVersions, setMcVersions] = useState<FabricMinecraftVersion[]>([]);
    const [loaderVersions, setLoaderVersions] = useState<FabricVersion[]>([]);
    useEffect(() => {
        invoke('get_fabric_minecraft_versions', { isQuilt: !!props.quilt })
            .then((obj) => {
                setMcVersions(obj as FabricMinecraftVersion[]);
                props.setSelectedMcVersion(
                    mcVersions.filter(
                        (mcVersion) => mcVersion.stable === stable
                    )[0].version
                );
            })
            .catch((e) => {});
        invoke('get_fabric_versions', { isQuilt: !!props.quilt })
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
            <img
                className='input-image'
                src={props.selectedVersion.length > 0 ? CheckIcon : AlertIcon}
                alt=''
            />
            <div className='forge-version-menu-container fabric'>
                <div className='forge-container minecraft-versions fabric'>
                    {mcVersions
                        .filter((mcVersion) => mcVersion.stable === stable)
                        .map((mcVersion, key) => (
                            <div
                                key={key}
                                className={`version clickable ${
                                    props.selectedMcVersion ===
                                    mcVersion.version
                                        ? 'selected'
                                        : ''
                                }`}
                                onClick={() => {
                                    props.setSelectedMcVersion(
                                        mcVersion.version
                                    );
                                    if (
                                        props.selectedMcVersion !==
                                        mcVersion.version
                                    ) {
                                        props.setSelectedVersion('');
                                    }
                                }}
                            >
                                <span>
                                    {props.selectedMcVersion ===
                                        mcVersion.version && (
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
                                props.selectedVersion === element.version
                                    ? 'selected'
                                    : ''
                            }`}
                            onClick={() => {
                                props.setSelectedVersion(element.version);
                            }}
                        >
                            <span>
                                {props.selectedVersion === element.version && (
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
