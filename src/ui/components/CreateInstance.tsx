import { invoke } from '@tauri-apps/api/tauri';
import React, { useState } from 'react';
import '../styles/CreateInstance.css';
import TextInput from './TextInput';
import VersionMenu from './VersionMenu';
import TextButton from './TextButton';
import ForgeVersionMenu from './ForgeVersionMenu';
import FabricVersionMenu from './FabricVersionMenu';

interface CreateInstanceProps {
    flavour: number | null;
    goToLibrary: () => void;
}

function CreateInstance(props: CreateInstanceProps): JSX.Element {
    const [titleInputValue, setTitleInputValue] = useState('');
    const [titleInputValid, setTitleInputValid] = useState(false);

    const [selectedVersionType, setSelectedVersionType] = useState(
        props.flavour === 0 ? 'release' : ''
    );
    const [selectedVersion, setSelectedVersion] = useState('');

    function handleTitleInputChange(
        event: React.ChangeEvent<HTMLInputElement>
    ): void {
        const { value } = event.target;
        setTitleInputValue(value);
        setTitleInputValid(
            // eslint-disable-next-line no-control-regex
            /^(?!^(?:(?:CON|PRN|AUX|NUL|COM\d|LPT\d)(?:\..*)?)$)[^<>:"/\\|?*\x00-\x1F]*[^<>:"/\\|?*\x00-\x1F .]$/i.test(
                value.trim()
            )
        );
    }

    return (
        <div className='create-instance'>
            <TextInput
                value={titleInputValue}
                onChange={handleTitleInputChange}
                name='Instance name'
                inputValid={titleInputValid}
            />
            {props.flavour === 0 && (
                <VersionMenu
                    autoScroll={false}
                    selectedVersionType={selectedVersionType}
                    selectedVersion={selectedVersion}
                    setSelectedVersionType={setSelectedVersionType}
                    setSelectedVersion={setSelectedVersion}
                />
            )}
            {props.flavour === 1 && (
                <ForgeVersionMenu
                    autoScroll={false}
                    selectedMcVersion={selectedVersionType}
                    selectedVersion={selectedVersion}
                    setSelectedMcVersion={setSelectedVersionType}
                    setSelectedVersion={setSelectedVersion}
                />
            )}
            {props.flavour === 2 && (
                <FabricVersionMenu
                    autoScroll={false}
                    selectedMcVersion={selectedVersionType}
                    selectedVersion={selectedVersion}
                    setSelectedMcVersion={setSelectedVersionType}
                    setSelectedVersion={setSelectedVersion}
                    quilt={false}
                />
            )}
            {props.flavour === 3 && (
                <FabricVersionMenu
                    autoScroll={false}
                    selectedMcVersion={selectedVersionType}
                    selectedVersion={selectedVersion}
                    setSelectedMcVersion={setSelectedVersionType}
                    setSelectedVersion={setSelectedVersion}
                    quilt={true}
                />
            )}
            <TextButton
                onClick={() => {
                    if (props.flavour === 0) {
                        invoke('create_instance', {
                            name: titleInputValue.trim(),
                            id: selectedVersion,
                            modloader: '',
                        }).catch((e) => {
                            console.log(e);
                        });
                    } else if (props.flavour === 1) {
                        invoke('create_instance', {
                            name: titleInputValue.trim(),
                            id: selectedVersionType,
                            modloader: 'forge-' + selectedVersion,
                        }).catch((e) => {
                            console.log(e);
                        });
                    } else if (props.flavour === 2) {
                        invoke('create_instance', {
                            name: titleInputValue.trim(),
                            id: selectedVersionType,
                            modloader: 'fabric-' + selectedVersion,
                        }).catch((e) => {
                            console.log(e);
                        });
                    } else if (props.flavour === 3) {
                        invoke('create_instance', {
                            name: titleInputValue.trim(),
                            id: selectedVersionType,
                            modloader: 'quilt-' + selectedVersion,
                        }).catch((e) => {
                            console.log(e);
                        });
                    }
                    props.goToLibrary();
                }}
                text='Create'
                clickable={titleInputValid && selectedVersion.length > 0}
            />
        </div>
    );
}

export default CreateInstance;
