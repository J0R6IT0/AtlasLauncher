import React, { useEffect, useRef, useState } from 'react';
import '../styles/ManageInstance.css';
import TextInput from './TextInput';
import TextButton from './TextButton';
import { type InstanceInfo } from '../../App';
import { invoke } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import InstanceBackground from '../../assets/images/instance-background.webp';

interface ManageInstanceProps {
    onClose: () => void
    target: Element | null
    updateInstances: () => void
}

function ManageInstance(props: ManageInstanceProps): JSX.Element {
    const [instanceName, setInstanceName] = useState('');
    const [titleInputValue, setTitleInputValue] = useState('');
    const [titleInputValid, setTitleInputValid] = useState(true);
    const [newBackground, setNewBackground] = useState('');
    const [instanceInfo, setInstanceInfo] = useState<InstanceInfo>();

    function handleTitleInputChange(event: React.ChangeEvent<HTMLInputElement>): void {
        const { value } = event.target;
        setTitleInputValue(value);
        // eslint-disable-next-line no-control-regex
        setTitleInputValid(/^(?!^(?:(?:CON|PRN|AUX|NUL|COM\d|LPT\d)(?:\..*)?)$)[^<>:"/\\|?*\x00-\x1F]*[^<>:"/\\|?*\x00-\x1F .]$/i.test(value.trim()));
    };

    const menuRef = useRef<HTMLDivElement>(null);
    const backgroundRef = useRef<HTMLImageElement>(null);

    const closeMenu = (): void => {
        menuRef.current?.classList.remove('visible');
        setTimeout(() => {
            props.onClose();
        }, 200);
    };

    const handleOutsideClick = (event: MouseEvent): void => {
        const menu = document.querySelector('.manage-instance') as HTMLElement;
        if (!menu.contains(event.target as Node)) {
            closeMenu();
        }
    };

    useEffect(() => {
        if (menuRef.current == null) {
            return;
        }
        const instanceName = props.target?.querySelector('span')?.innerText;
        if (instanceName !== undefined) {
            setInstanceName(instanceName);
            setTitleInputValue(instanceName);
            invoke('read_instance_data', { name: instanceName }).then((info): void => {
                setInstanceInfo(info as InstanceInfo);
            }).catch(e => {});
        };

        setTimeout(() => {
            menuRef.current?.classList.add('visible');
            document.addEventListener('click', handleOutsideClick);
        }, 10);

        return () => {
            document.removeEventListener('click', handleOutsideClick);
        };
    }, []);

    return (
        <div className='manage-instance-container'>
            <div ref={menuRef} className='manage-instance'>
                <div className='manage-instance-title'><span>{instanceName}</span></div>
                <div className='manage-instance-side'>
                    <div className='manage-instance-background clickable' onClick={() => {
                        open({
                            multiple: false,
                            filters: [{
                                name: 'Instance Background',
                                extensions: ['png', 'jpeg', 'webp', 'gif']
                            }]
                        }).then((selected) => {
                            if (selected !== null && !Array.isArray(selected)) {
                                backgroundRef.current?.setAttribute('src', convertFileSrc(selected));
                                setNewBackground(selected);
                            }
                        }).catch((e) => {});
                    }}>
                        <img ref={backgroundRef} src={instanceInfo?.background !== undefined && instanceInfo?.background.length > 0 ? convertFileSrc(instanceInfo.background) : InstanceBackground}/>
                    </div>
                    <TextButton text='Apply Changes' onClick={() => {
                        invoke('write_instance_data', { name: instanceName, newName: titleInputValue, version: instanceInfo?.version, background: newBackground }).then(() => {
                            props.updateInstances();
                            closeMenu();
                        }).catch(e => {});
                    }} clickable={titleInputValid && (titleInputValue !== instanceName || newBackground.length > 0)}/>
                </div>
                <div className='manage-instance-fields'>
                    <TextInput value={titleInputValue} onChange={handleTitleInputChange} name='Instance name' inputValid={titleInputValid}/>
                </div>
            </div>
        </div>
    );
}

export default ManageInstance;
