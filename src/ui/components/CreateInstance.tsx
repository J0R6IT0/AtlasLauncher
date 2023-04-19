import { invoke } from '@tauri-apps/api/tauri';
import React, { useState } from 'react';
import '../styles/CreateInstance.css';
import TextInput from './TextInput';
import VersionMenu from './VersionMenu';

interface CreateInstanceProps {
    flavour: number | null
    goToLibrary: () => void
}

function CreateInstance(props: CreateInstanceProps): JSX.Element {
    const [titleInputValue, setTitleInputValue] = useState('');
    const [titleInputValid, setTitleInputValid] = useState(false);

    const [selectedVersionType, setSelectedVersionType] = useState('release');
    const [selectedVersion, setSelectedVersion] = useState('');

    function handleTitleInputChange(event: React.ChangeEvent<HTMLInputElement>): void {
        const { value } = event.target;
        setTitleInputValue(value);
        // eslint-disable-next-line no-control-regex
        setTitleInputValid(/^(?!^(?:(?:CON|PRN|AUX|NUL|COM\d|LPT\d)(?:\..*)?)$)[^<>:"/\\|?*\x00-\x1F]*[^<>:"/\\|?*\x00-\x1F .]$/i.test(value.trim()));
    };

    return (
        <div className='create-instance'>
            <TextInput value={titleInputValue} onChange={handleTitleInputChange} name='Instance name' inputValid={titleInputValid}/>
            <VersionMenu selectedVersionType={selectedVersionType} selectedVersion={selectedVersion} setSelectedVersionType={setSelectedVersionType} setSelectedVersion={setSelectedVersion}/>
            <div className={`create-button ${titleInputValid && selectedVersion.length > 0 ? 'valid' : ''}`} onClick={() => {
                if (titleInputValid && selectedVersion.length > 0) {
                    invoke('create_instance', { name: titleInputValue.trim(), version: selectedVersion, versionType: selectedVersionType }).catch(e => { console.log(e); });
                    props.goToLibrary();
                }
            }}><span>Create</span></div>
        </div>
    );
}

export default CreateInstance;
