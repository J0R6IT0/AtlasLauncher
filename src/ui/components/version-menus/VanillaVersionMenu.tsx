import { For, createResource, type JSX, createSignal, Show } from 'solid-js';
import './styles.scss';
import { TickIcon, WarnIcon } from '../../../assets/icons/Icons';
import { invoke } from '@tauri-apps/api/tauri';
import { vanillaVersionTypes } from '../../../data/constants';
import { type VersionMenuProps } from '../../pages/NewInstance';

interface MinecraftVersion {
    id: string;
    url: string;
    sha1: string;
    releaseTime: string;
    type: string;
}

function VanillaVersionMenu(props: VersionMenuProps): JSX.Element {
    const fetchVersions = async (): Promise<MinecraftVersion[]> => {
        return (await invoke(
            'update_minecraft_version_manifest',
        ).catch()) as MinecraftVersion[];
    };

    const [versions] = createResource(fetchVersions);

    const [versionType, setVersionType] = createSignal('release');

    return (
        <div class='version-menu'>
            <div class='version-menu-wrapper'>
                <div class='version-menu-tabs'>
                    <For each={vanillaVersionTypes}>
                        {(item) => (
                            <span
                                classList={{
                                    selected: versionType() === item.type,
                                }}
                                class='clickable'
                                onClick={() => setVersionType(item.type)}
                            >
                                {item.name}
                            </span>
                        )}
                    </For>
                </div>
                <div class='version-container-wrapper'>
                    <div class='version-container'>
                        <For
                            each={versions()?.filter(
                                (version) => version.type === versionType(),
                            )}
                        >
                            {(version) => (
                                <div
                                    onClick={() => {
                                        props.setSelectedVersion(version.id);
                                    }}
                                    classList={{
                                        selected:
                                            version.id ===
                                            props.selectedVersion,
                                    }}
                                    class='version clickable'
                                >
                                    <Show
                                        when={
                                            props.selectedVersion === version.id
                                        }
                                    >
                                        <div class='indicator' />
                                    </Show>
                                    <span>{version.id}</span>
                                </div>
                            )}
                        </For>
                    </div>
                </div>
            </div>
            {props.selectedVersion.length > 0 ? <TickIcon /> : <WarnIcon />}
        </div>
    );
}

export default VanillaVersionMenu;
