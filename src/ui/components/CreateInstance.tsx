import { invoke } from '@tauri-apps/api/tauri';
import React, { useState } from 'react';
import '../styles/CreateInstance.css';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import TextInput from './TextInput';

const releaseArray: string[] = await invoke('list_minecraft_versions', { versionType: 'release' });
const snapshotArray: string[] = await invoke('list_minecraft_versions', { versionType: 'snapshot' });
const oldBetaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_beta' });
const oldAlphaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_alpha' });

interface CreateInstanceProps {
    flavour: number | null
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
            <div className='create-instance-version'>
                <div className='version-tabs'>
                    <div className={`version-type ${selectedVersionType === 'release' ? 'selected' : ''}`} onClick={() => { setSelectedVersionType('release'); }}><span>Release</span></div>
                    <div className={`version-type ${selectedVersionType === 'snapshot' ? 'selected' : ''}`} onClick={() => { setSelectedVersionType('snapshot'); }}><span>Snapshot</span></div>
                    <div className={`version-type ${selectedVersionType === 'old_beta' ? 'selected' : ''}`} onClick={() => { setSelectedVersionType('old_beta'); }}><span>Beta</span></div>
                    <div className={`version-type ${selectedVersionType === 'old_alpha' ? 'selected' : ''}`} onClick={() => { setSelectedVersionType('old_alpha'); }}><span>Alpha</span></div>
                </div>
                <img className="input-image" src={selectedVersion.length > 0 ? CheckIcon : AlertIcon} alt="" />
                <div className='version-container'>
                    {selectedVersionType === 'release' && releaseArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                    {selectedVersionType === 'snapshot' && snapshotArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                    {selectedVersionType === 'old_beta' && oldBetaArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                    {selectedVersionType === 'old_alpha' && oldAlphaArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                </div>
            </div>
            <div className={`create-button ${titleInputValid && selectedVersion.length > 0 ? 'valid' : ''}`} onClick={() => {
                if (titleInputValid && selectedVersion.length > 0) {
                    invoke('create_instance', { name: titleInputValue.trim(), version: selectedVersion, versionType: selectedVersionType }).catch(e => { console.log(e); });
                    setTitleInputValid(false);
                    setTitleInputValue('');
                    setSelectedVersionType('release');
                    setSelectedVersion('');
                }
            }}><span>Create</span></div>
        </div>
    );
}

export default CreateInstance;
