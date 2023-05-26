import React, { memo, useState } from 'react';
import '../styles/NewInstance.css';
import MinecraftForge from '../../assets/images/minecraft-forge.webp';
import MinecraftFabric from '../../assets/images/minecraft-fabric.webp';
import MinecraftQuilt from '../../assets/images/minecraft-quilt.webp';
import MinecraftCover from '../../assets/images/minecraft-cover.webp';
import CreateInstance from '../components/CreateInstance';

import {
    BoxIcon,
    FabricIcon,
    ForgeIcon,
    QuiltIcon,
} from '../../assets/icons/Icons';

export enum Flavours {
    Vanilla,
    Forge,
    Fabric,
    Quilt,
}

export const flavours = [
    {
        id: Flavours.Vanilla,
        name: 'Vanilla',
        background: MinecraftCover,
        icon: BoxIcon,
    },
    {
        id: Flavours.Forge,
        name: 'Forge',
        background: MinecraftForge,
        icon: ForgeIcon,
    },
    {
        id: Flavours.Fabric,
        name: 'Fabric',
        background: MinecraftFabric,
        icon: FabricIcon,
    },
    {
        id: Flavours.Quilt,
        name: 'Quilt',
        background: MinecraftQuilt,
        icon: QuiltIcon,
    },
];

interface NewInstanceProps {
    goToLibrary: () => void;
}

const NewInstance = React.memo((props: NewInstanceProps) => {
    const [selectedFlavour, setSelectedFlavour] = useState<number | null>(null);
    return (
        <React.Fragment>
            <div className='flavour-container'>
                {flavours.map((element, index) => (
                    <div
                        key={index}
                        className={`flavour ${
                            selectedFlavour === element.id ? 'selected' : ''
                        }`}
                        onClick={() => {
                            setSelectedFlavour(element.id);
                        }}
                    >
                        <img
                            className='flavour-background'
                            src={element.background}
                        />
                        <span className='flavour-icon'>
                            <element.icon />
                            {element.name}
                        </span>
                        <div className='flavour-wrapper'>
                            <div className='flavour-data'>
                                <element.icon />
                                <span>{element.name}</span>
                            </div>
                            <div className='flavour-content'>
                                {selectedFlavour === element.id && (
                                    <CreateInstance
                                        flavour={selectedFlavour}
                                        goToLibrary={props.goToLibrary}
                                    />
                                )}
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </React.Fragment>
    );
});

NewInstance.displayName = 'NewInstance';

export default memo(NewInstance);
