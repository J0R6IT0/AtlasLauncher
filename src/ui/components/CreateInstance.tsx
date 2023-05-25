import { invoke } from '@tauri-apps/api/tauri';
import React, { useState } from 'react';
import TextInput from './TextInput';
import VersionMenu from './VersionMenu';
import TextButton from './TextButton';
import ForgeVersionMenu from './ForgeVersionMenu';
import FabricVersionMenu from './FabricVersionMenu';
import { Flavours } from '../pages/NewInstance';

interface CreateInstanceProps {
    flavour: Flavours | null;
    goToLibrary: () => void;
}

function CreateInstance(props: CreateInstanceProps): JSX.Element {
    const [titleInputValue, setTitleInputValue] = useState('');
    const [titleInputValid, setTitleInputValid] = useState(false);

    const [mcVersion, setMcVersion] = useState<string>('');
    const [modloaderVersion, setModloaderVersion] = useState<string>('');

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

    const menuProps = {
        autoScroll: false,
        mcVersion,
        setMcVersion,
        modloaderVersion,
        setModloaderVersion,
    };

    return (
        <React.Fragment>
            <TextInput
                value={titleInputValue}
                onChange={handleTitleInputChange}
                name='Instance name'
                inputValid={titleInputValid}
            />
            {props.flavour === Flavours.Vanilla && (
                <VersionMenu {...menuProps} />
            )}
            {props.flavour === Flavours.Forge && (
                <ForgeVersionMenu {...menuProps} />
            )}
            {props.flavour === Flavours.Fabric && (
                <FabricVersionMenu {...menuProps} isQuilt={false} />
            )}
            {props.flavour === Flavours.Quilt && (
                <FabricVersionMenu {...menuProps} isQuilt={true} />
            )}
            <TextButton
                onClick={() => {
                    let prefix = '';

                    switch (props.flavour) {
                        case Flavours.Forge:
                            prefix = 'forge-';
                            break;
                        case Flavours.Fabric:
                            prefix = 'fabric-';
                            break;
                        case Flavours.Quilt:
                            prefix = 'quilt-';
                            break;
                    }
                    invoke('create_instance', {
                        name: titleInputValue.trim(),
                        id: mcVersion,
                        modloader: prefix + modloaderVersion,
                    }).catch((e) => {});
                    props.goToLibrary();
                }}
                text='Create'
                clickable={
                    titleInputValid &&
                    ((props.flavour !== Flavours.Vanilla &&
                        modloaderVersion.length > 0) ||
                        (props.flavour === Flavours.Vanilla &&
                            mcVersion.length > 0))
                }
            />
        </React.Fragment>
    );
}

export default CreateInstance;
