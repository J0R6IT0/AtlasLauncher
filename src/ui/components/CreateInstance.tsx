import { invoke } from '@tauri-apps/api/tauri';
import React, { useState, useEffect } from 'react';
import '../styles/CreateInstance.css';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';
import { listen } from '@tauri-apps/api/event';
import toast from 'react-hot-toast';

const releaseArray: string[] = await invoke('list_minecraft_versions', { versionType: 'release' });
const snapshotArray: string[] = await invoke('list_minecraft_versions', { versionType: 'snapshot' });
const oldBetaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_beta' });
const oldAlphaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_alpha' });

interface CreateInstanceProps {
    flavour: number | null
    setFlavour: (flavour: number | null) => void
}

interface CreateInstanceEvent {
    payload: CreateInstanceEventPayload
}

interface CreateInstanceEventPayload {
    status: string
    message: string
    name: string
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

    useEffect(() => {
        listen('create_instance', (event: CreateInstanceEvent) => {
            if (event.payload.status === 'Success') {
                toast.success(event.payload.message, {
                    id: event.payload.name,
                    duration: 6000,
                    position: 'bottom-center',
                    iconTheme: {
                        primary: 'var(--icons-color)',
                        secondary: 'var(--icons-color-hover)'
                    }
                });
            } else if (event.payload.status === 'Error') {
                toast.error(event.payload.message, {
                    id: event.payload.name,
                    duration: 10000,
                    position: 'bottom-center',
                    iconTheme: {
                        primary: 'var(--icons-color)',
                        secondary: 'var(--icons-color-hover)'
                    }
                });
            } else if (event.payload.status === 'Loading') {
                toast.loading(event.payload.message, {
                    id: event.payload.name,
                    position: 'bottom-center',
                    className: 'toast-notification',
                    iconTheme: {
                        primary: 'var(--icons-color-hover)',
                        secondary: 'var(--icons-color)'
                    }
                });
            } else {
                toast.dismiss(event.payload.name);
            }
        }).catch(e => {});
    });

    return (
        <div className='create-instance'>
            <div className='input'>
                <input type="text" value={titleInputValue} required spellCheck='false' onChange={handleTitleInputChange} maxLength={32} title=''/>
                <span className="floating-input-label">Instance Name</span>
                <img className="input-image" src={titleInputValid ? CheckIcon : AlertIcon} alt="" />
            </div>
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
