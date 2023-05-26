import React, { useEffect, useRef, useState } from 'react';
import '../styles/ManageInstance.css';
import TextInput from './TextInput';
import TextButton from './TextButton';
import { type InstanceInfo } from '../../App';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { Instance, defaultBackgrounds, defaultIcons } from '../pages/Library';
import VersionMenu from './VersionMenu';
import BaseDropdown from './BaseDropdown';
import BaseToggle from './BaseToggle';
import mountAnimationHandler from '../../utils/mountAnimationHandler';
import {
    PlusIcon,
    GlobeIcon,
    CoffeeIcon,
    PenToolIcon,
} from '../../assets/icons/Icons';

const resolutions = [
    '3840x2160',
    '2560x1600',
    '2560x1440',
    '1920x1200',
    '1920x1080',
    '1680x1050',
    '1600x900',
    '1440x900',
    '1366x768',
    '1280x800',
    '1280x720',
    '1024x768',
    '800x600',
    '640x480',
];

interface ManageInstanceProps {
    onClose: () => void;
    target: InstanceInfo | null;
    updateInstances: () => void;
}

interface GeneralProps extends BaseManageInstancePageProps {
    titleInputValid: boolean;
    titleInputValue: string;
    handleTitleInputChange: (
        event: React.ChangeEvent<HTMLInputElement>
    ) => void;
}

interface AppearanceProps extends BaseManageInstancePageProps {}

interface JavaProps extends BaseManageInstancePageProps {}

interface BaseManageInstancePageProps {
    changeProperty: (
        propertyName: string,
        propertyValue: string | boolean
    ) => void;
    instanceInfo: InstanceInfo | undefined;
}

enum Categories {
    General,
    Java,
    Appearance,
}

const categories = [
    { category: Categories.General, name: 'General', icon: GlobeIcon },
    { category: Categories.Java, name: 'Java', icon: CoffeeIcon },
    { category: Categories.Appearance, name: 'Appearance', icon: PenToolIcon },
];

function ManageInstance(props: ManageInstanceProps): JSX.Element {
    const [instanceName, setInstanceName] = useState('');
    const [category, setCategory] = useState(Categories.General);
    const [instanceInfo, setInstanceInfo] = useState<InstanceInfo>();
    const [titleInputValue, setTitleInputValue] = useState('');
    const [titleInputValid, setTitleInputValid] = useState(true);
    const [wasValueModified, setWasValueModified] = useState(false);

    const menuRef = useRef<HTMLDivElement>(null);

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
        handlePropertyChange('name', value);
    }

    const handleClose = (): void => {
        menuRef.current?.classList.remove('visible');
        setTimeout(() => {
            props.onClose();
        }, 300);
    };

    mountAnimationHandler(menuRef, handleClose);

    const handlePropertyChange = (
        propertyName: string,
        propertyValue: string | boolean
    ): void => {
        if (
            instanceInfo !== undefined &&
            !Object.prototype.hasOwnProperty.call(instanceInfo, propertyName)
        ) {
            return;
        }
        setWasValueModified(true);
        const newInstanceInfo: InstanceInfo = instanceInfo as InstanceInfo;
        newInstanceInfo[propertyName] = propertyValue;
        setInstanceInfo(newInstanceInfo);
    };

    useEffect(() => {
        if (menuRef.current == null) {
            return;
        }
        if (props.target !== null) {
            setInstanceName(props.target.name);
            setTitleInputValue(props.target.name);
            setInstanceInfo({ ...props.target });
        }
    }, []);

    const pageProps = {
        changeProperty: (
            propertyName: string,
            propertyValue: string | boolean
        ) => {
            handlePropertyChange(propertyName, propertyValue);
        },
        instanceInfo,
        titleInputValid,
        titleInputValue,
        handleTitleInputChange,
    };

    return (
        <div className='manage-instance-container'>
            <div className='manage-instance' ref={menuRef}>
                <div className='manage-instance-title'>
                    <span>{instanceName}</span>
                </div>
                <div className='manage-instance-categories'>
                    {categories.map((element, key) => (
                        <div
                            key={key}
                            className={`instance-category clickable hover accent-text-secondary ${
                                category === element.category ? 'active' : ''
                            }`}
                            onClick={() => {
                                setCategory(element.category);
                            }}
                        >
                            <element.icon />
                            <span>{element.name}</span>
                            <hr />
                        </div>
                    ))}
                    <TextButton
                        text='Save'
                        clickable={titleInputValid && wasValueModified}
                        onClick={() => {
                            handleClose();
                            invoke('write_instance_data', {
                                name: instanceName,
                                data: instanceInfo,
                            })
                                .then(() => {
                                    props.updateInstances();
                                })
                                .catch((e) => {});
                        }}
                    />
                </div>
                {category === Categories.General && <General {...pageProps} />}
                {category === Categories.Java && <Java {...pageProps} />}
                {category === Categories.Appearance && (
                    <Appearance {...pageProps} />
                )}
            </div>
        </div>
    );
}

export default ManageInstance;

function General(props: GeneralProps): JSX.Element {
    if (props.instanceInfo !== undefined) {
        const [selectedVersion, setSelectedVersion] = useState(
            props.instanceInfo.version
        );
        return (
            <div className='manage-instance-fields'>
                <TextInput
                    value={props.titleInputValue}
                    onChange={props.handleTitleInputChange}
                    name='Instance name'
                    inputValid={props.titleInputValid}
                />
                <VersionMenu
                    autoScroll={true}
                    mcVersion={selectedVersion}
                    setMcVersion={(version) => {
                        props.changeProperty('version', version);
                        setSelectedVersion(version);
                    }}
                />
            </div>
        );
    }
    return <div className='manage-instance-fields'></div>;
}

function Java(props: JavaProps): JSX.Element {
    if (props.instanceInfo !== undefined) {
        const [isFullscreenEnabled, setFullscreenEnabled] = useState(
            props.instanceInfo.fullscreen
        );
        return (
            <div className='manage-instance-fields'>
                <div className='resolution-wrapper'>
                    <BaseDropdown
                        onSelect={(value) => {
                            const resolution = value.split('x');
                            props.changeProperty('width', resolution[0]);
                            props.changeProperty('height', resolution[1]);
                        }}
                        autoScroll={true}
                        default={
                            props.instanceInfo.width +
                            'x' +
                            props.instanceInfo.height
                        }
                        values={resolutions}
                        placeholder='Resolution'
                    />
                </div>
                <div
                    className={`option ${isFullscreenEnabled ? 'enabled' : ''}`}
                >
                    <BaseToggle
                        default={isFullscreenEnabled}
                        onEnable={() => {
                            setFullscreenEnabled(true);
                            props.changeProperty('fullscreen', true);
                        }}
                        onDisable={() => {
                            setFullscreenEnabled(false);
                            props.changeProperty('fullscreen', false);
                        }}
                    />
                    <span>Fullscreen</span>
                </div>
            </div>
        );
    }
    return <div className='manage-instance-fields'></div>;
}

function Appearance(props: AppearanceProps): JSX.Element {
    const [newBackground, setNewBackground] = useState('');
    const [newIcon, setNewIcon] = useState('');
    if (props.instanceInfo !== undefined) {
        return (
            <div className='manage-instance-fields appearance'>
                <div className='instance-preview'>
                    <Instance
                        key={newBackground + newIcon}
                        element={props.instanceInfo}
                        handleContextMenu={() => {}}
                        onClick={() => {}}
                    />
                </div>
                <div className='backgrounds'>
                    <span>Background</span>
                    <div className='default-backgrounds'>
                        <div
                            className='default-background-wrapper clickable hover accent-text-secondary'
                            onClick={() => {
                                open({
                                    multiple: false,
                                    filters: [
                                        {
                                            name: 'Instance Background',
                                            extensions: [
                                                'png',
                                                'jpeg',
                                                'webp',
                                                'gif',
                                            ],
                                        },
                                    ],
                                })
                                    .then((selected) => {
                                        if (
                                            selected !== null &&
                                            !Array.isArray(selected)
                                        ) {
                                            setNewBackground(selected);
                                            props.changeProperty(
                                                'background',
                                                selected
                                            );
                                        }
                                    })
                                    .catch((e) => {});
                            }}
                        >
                            <PlusIcon />
                        </div>
                        {defaultBackgrounds.map((element, key) => (
                            <div
                                key={key}
                                className='default-background-wrapper clickable hover'
                                onClick={() => {
                                    setNewBackground(`default${key}`);
                                    props.changeProperty(
                                        'background',
                                        `default${key}`
                                    );
                                }}
                            >
                                <img
                                    className='default-background'
                                    src={defaultBackgrounds[key]}
                                />
                            </div>
                        ))}
                    </div>
                </div>
                <div className='icons'>
                    <span>Icon</span>
                    <div className='default-icons'>
                        <div
                            className='default-icon-wrapper new clickable hover accent-text-secondary'
                            onClick={() => {
                                open({
                                    multiple: false,
                                    filters: [
                                        {
                                            name: 'Instance Icon',
                                            extensions: [
                                                'png',
                                                'jpeg',
                                                'webp',
                                                'gif',
                                            ],
                                        },
                                    ],
                                })
                                    .then((selected) => {
                                        if (
                                            selected !== null &&
                                            !Array.isArray(selected)
                                        ) {
                                            setNewIcon(selected);
                                            props.changeProperty(
                                                'icon',
                                                selected
                                            );
                                        }
                                    })
                                    .catch((e) => {});
                            }}
                        >
                            <PlusIcon />
                        </div>
                        {defaultIcons.map((element, key) => (
                            <div
                                key={key}
                                className='default-icon-wrapper clickable hover'
                                onClick={() => {
                                    setNewIcon(`default${key}`);
                                    props.changeProperty(
                                        'icon',
                                        `default${key}`
                                    );
                                }}
                            >
                                <img
                                    className='default-icon'
                                    src={defaultIcons[key]}
                                />
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        );
    }
    return <div className='manage-instance-fields'></div>;
}
