import { invoke } from '@tauri-apps/api/tauri';
import React, { useState, useEffect, useRef } from 'react';
import '../styles/CreateInstance.css';
import MinecraftDefault from '../../assets/images/minecraft-default.png';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';

const releaseArray: string[] = await invoke('list_minecraft_versions', { versionType: 'release' });
const snapshotArray: string[] = await invoke('list_minecraft_versions', { versionType: 'snapshot' });
const oldBetaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_beta' });
const oldAlphaArray: string[] = await invoke('list_minecraft_versions', { versionType: 'old_alpha' });

interface CreateInstanceProps {
    createInstance: string
    setCreateInstance: (flavour: string) => void
}

function CreateInstance(props: CreateInstanceProps): JSX.Element {
    const [maxDropdownHeight, setMaxDropdownHeight] = useState(0);

    const [selectedVersionType, setSelectedVersionType] = useState(0);
    const [selectedVersion, setSelectedVersion] = useState('');

    const [titleInputValue, setTitleInputValue] = useState('');
    const [titleInputValid, setTitleInputValid] = useState(false);

    const containerRef = useRef<HTMLDivElement>(null);
    const dropdownRef = useRef<HTMLDivElement>(null);

    function handleTitleInputChange(event: React.ChangeEvent<HTMLInputElement>): void {
        const { value } = event.target;
        setTitleInputValue(value);
        setTitleInputValid(/^\s*(?! )[\s\S]{0,32}[^\s](?<! )\s*$/i.test(value));
    };

    useEffect(() => {
        function handleResize(): void {
            const containerRefOffset = containerRef.current?.offsetHeight === undefined ? null : containerRef.current.offsetHeight;
            const newMaxHeight = containerRefOffset === null ? 0 : containerRefOffset - 161;
            setMaxDropdownHeight(newMaxHeight);
        }
        handleResize();
        window.addEventListener('resize', handleResize);
        return () => { window.removeEventListener('resize', handleResize); };
    }, [containerRef, dropdownRef]);

    return (
        <div className='create-instance' onClick={() => { props.setCreateInstance(''); }}>
            <div className='create-instance-modal' onClick={(e) => { e.stopPropagation(); }} ref={containerRef}>
                <div className='create-instance-title'>
                    <span>New Instance - Vanilla</span>
                </div>
                <div className='create-instance-body'>
                    <div className='create-instance-side-bar'>
                        <div className='create-instance-image-container'>
                            <img className='create-instance-image' src={MinecraftDefault} alt="" />
                        </div>
                        <div className='create-instance-create'>
                            <span>Create Instance</span>
                        </div>
                    </div>
                    <div className='create-instance-fields'>
                        <div className='input'>
                            <input type="text" value={titleInputValue} required spellCheck='false' onChange={handleTitleInputChange} maxLength={32} title=''/>
                            <span className="floating-input-label">Instance Name</span>
                            <img className="input-image" src={titleInputValid ? CheckIcon : AlertIcon} alt="" />

                        </div>
                        <div className='create-instance-version'>
                            <div className='version-tabs'>
                                <div className={`version-type ${selectedVersionType === 0 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(0); }}><span>Releases</span></div>
                                <div className={`version-type ${selectedVersionType === 1 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(1); }}><span>Snapshots</span></div>
                                <div className={`version-type ${selectedVersionType === 2 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(2); }}><span>Betas</span></div>
                                <div className={`version-type ${selectedVersionType === 3 ? 'selected' : ''}`} onClick={() => { setSelectedVersionType(3); }}><span>Alphas</span></div>
                            </div>
                            <img className="input-image" src={selectedVersion.length > 0 ? CheckIcon : AlertIcon} alt="" />
                            <div className='version-container' ref={dropdownRef} style={{ maxHeight: `${maxDropdownHeight}px` }}>
                                {selectedVersionType === 0 && releaseArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{element}</span></div>)}
                                {selectedVersionType === 1 && snapshotArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{element}</span></div>)}
                                {selectedVersionType === 2 && oldBetaArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{element}</span></div>)}
                                {selectedVersionType === 3 && oldAlphaArray.map((element, index) => <div key={index} className={`version ${selectedVersion === element ? 'selected' : ''}`} onClick={() => { setSelectedVersion(element); } }><span>{element}</span></div>)}

                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default CreateInstance;
