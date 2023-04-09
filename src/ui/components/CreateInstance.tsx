import { invoke } from '@tauri-apps/api/tauri';
import React, { useState } from 'react';
import '../styles/CreateInstance.css';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';

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

    const [selectedVersionType, setSelectedVersionType] = useState(0);
    const [selectedVersion, setSelectedVersion] = useState('');

    function handleTitleInputChange(event: React.ChangeEvent<HTMLInputElement>): void {
        const { value } = event.target;
        setTitleInputValue(value);
        // eslint-disable-next-line no-control-regex
        setTitleInputValid(/^(?!^(?:(?:CON|PRN|AUX|NUL|COM\d|LPT\d)(?:\..*)?)$)[^<>:"/\\|?*\x00-\x1F]*[^<>:"/\\|?*\x00-\x1F .]$/i.test(value.trim()));
    };

    return (
        <div className='create-instance'>
            <div className='input'>
                <input type="text" value={titleInputValue} required spellCheck='false' onChange={handleTitleInputChange} maxLength={32} title=''/>
                <span className="floating-input-label">Instance Name</span>
                <img className="input-image" src={titleInputValid ? CheckIcon : AlertIcon} alt="" />
            </div>
            <div className='create-instance-version'>
                <div className='version-tabs'>
                    <div className={`version-type ${selectedVersionType === 0 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(0); }}><span>Release</span></div>
                    <div className={`version-type ${selectedVersionType === 1 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(1); }}><span>Snapshot</span></div>
                    <div className={`version-type ${selectedVersionType === 2 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(2); }}><span>Beta</span></div>
                    <div className={`version-type ${selectedVersionType === 3 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(3); }}><span>Alpha</span></div>
                </div>
                <img className="input-image" src={selectedVersion.length > 0 ? CheckIcon : AlertIcon} alt="" />
                <div className='version-container'>
                    {selectedVersionType === 0 && releaseArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                    {selectedVersionType === 1 && snapshotArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                    {selectedVersionType === 2 && oldBetaArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                    {selectedVersionType === 3 && oldAlphaArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{selectedVersion === element && <div className='dot'></div>}{element}</span></div>)}
                </div>
            </div>
            <div className={`create-button ${titleInputValid && selectedVersion.length > 0 ? 'valid' : ''}`}><span>Create</span></div>
        </div>
    );
}

export default CreateInstance;
