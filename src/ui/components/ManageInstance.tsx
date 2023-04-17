import React, { useEffect, useRef, useState } from 'react';
import '../styles/ManageInstance.css';
import TextInput from './TextInput';
import { type InstanceInfo } from '../../App';
import { invoke } from '@tauri-apps/api';

interface ManageInstanceProps {
    onClose: () => void
    target: Element | null
    updateInstances: () => void
}

function ManageInstance(props: ManageInstanceProps): JSX.Element {
    const [instanceName, setInstanceName] = useState('');
    const [titleInputValue, setTitleInputValue] = useState('');
    const [titleInputValid, setTitleInputValid] = useState(true);
    const [instanceInfo, setInstanceInfo] = useState<InstanceInfo>();

    function handleTitleInputChange(event: React.ChangeEvent<HTMLInputElement>): void {
        const { value } = event.target;
        setTitleInputValue(value);
        // eslint-disable-next-line no-control-regex
        setTitleInputValid(/^(?!^(?:(?:CON|PRN|AUX|NUL|COM\d|LPT\d)(?:\..*)?)$)[^<>:"/\\|?*\x00-\x1F]*[^<>:"/\\|?*\x00-\x1F .]$/i.test(value.trim()));
    };

    const menuRef = useRef<HTMLDivElement>(null);

    const closeMenu = (): void => {
        setTimeout(() => {
            props.onClose();
        }, 200);
    };

    const handleOutsideClick = (event: MouseEvent): void => {
        const menu = document.querySelector('.manage-instance') as HTMLElement;
        if (!menu.contains(event.target as Node)) {
            menuRef.current?.classList.remove('visible');
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
                <div className='manage-instance-title'><span>{props.target?.querySelector('span')?.innerText}</span></div>
                <div className='manage-instance-side'>
                    <div className={`manage-instance-apply ${titleInputValid && titleInputValue !== instanceName ? 'valid' : ''}`} onClick={() => {
                        invoke('write_instance_data', { name: instanceName, newName: titleInputValue, version: instanceInfo?.version }).then(() => {
                            props.updateInstances();
                            closeMenu();
                        }).catch(e => {});
                    }}>Apply Changes</div>
                </div>
                <div className='manage-instance-fields'>
                    <TextInput value={titleInputValue} onChange={handleTitleInputChange} name='Instance name' inputValid={titleInputValid}/>
                </div>
            </div>
        </div>
    );
}

export default ManageInstance;
