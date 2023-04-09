import React, { useState } from 'react';
import '../styles/NewInstance.css';
import MinecraftForge from '../../assets/images/minecraft-forge.jpg';
import MinecraftCover from '../../assets/images/minecraft-cover.png';
import CreateInstance from '../components/CreateInstance';
import BoxIcon from '../../assets/icons/box.svg';
import ForgeIcon from '../../assets/icons/forge.svg';
import FabricIcon from '../../assets/icons/fabric.svg';

const flavours = [
    { id: 0, name: 'Vanilla', background: MinecraftCover, icon: BoxIcon },
    { id: 1, name: 'Forge', background: MinecraftForge, icon: ForgeIcon },
    { id: 2, name: 'Fabric', background: 'https://zonacraft.net/wp-content/uploads/2022/07/Eden-Ring-Mod.png', icon: FabricIcon }
];

function NewInstance(): JSX.Element {
    const [selectedFlavour, setSelectedFlavour] = useState<number | null>(null);
    const [createInstance, setCreateInstance] = useState('');
    function showCreateInstance(flavour: string): void {
        setCreateInstance(flavour);
    }

    return (
        <div className='new-instance'>
            <div className='new-instance-info'>
                <span className='new-instance-title'>New Instance</span>
                <span>Add a new instance to your library</span>
            </div>
            <div className='flavour-container'>
                { flavours.map((element, index) => <div key={index} className={`flavour ${selectedFlavour === element.id ? 'selected' : ''}`} onClick={() => {
                    setSelectedFlavour(element.id);
                }}>
                    <img className='flavour-background' src={element.background} alt="" />
                    <span className='flavour-icon' ><img src={element.icon} className={`${element.name === 'Fabric' ? 'fabric-icon' : ''}`} alt="" />{element.name}</span>
                    <div className='flavour-wrapper'>
                        <div className='flavour-data'>
                            <img className={`${element.name === 'Fabric' ? 'fabric-icon' : ''}`} src={element.icon} alt="" />
                            <span>{element.name}</span>
                        </div>
                    </div>
                </div>)}
            </div>
        </div>
    );
}

export default NewInstance;
