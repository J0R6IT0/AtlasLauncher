import React, { useState } from 'react';
import '../styles/NewInstance.css';
import MinecraftForge from '../../assets/images/minecraft-forge.jpg';
import MinecraftCover from '../../assets/images/minecraft-cover.png';
import CreateInstance from '../components/CreateInstance';
import BoxIcon from '../../assets/icons/box.svg';
import ForgeIcon from '../../assets/icons/forge.svg';
import FabricIcon from '../../assets/icons/fabric.svg';

function NewInstance(): JSX.Element {
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
                <div className='flavour'>
                    <img className='flavour-background' src={MinecraftCover} alt="" />
                    <span className='flavour-icon' ><img src={BoxIcon} alt="" />Vanilla</span>
                    <div className='flavour-wrapper'>
                        <img src={BoxIcon} alt="" />
                        <span>Vanilla</span>
                    </div>
                </div>
                <div className='flavour'>
                    <img className='flavour-background' src={MinecraftForge} alt="" />
                    <span className='flavour-icon' ><img src={ForgeIcon} alt="" />Forge</span>
                    <div className='flavour-wrapper'>
                        <img src={ForgeIcon} alt="" />
                        <span>Forge</span>
                    </div>
                </div>
                <div className='flavour'>
                    <img className='flavour-background' src="https://zonacraft.net/wp-content/uploads/2022/07/Eden-Ring-Mod.png" alt="" />
                    <span className='flavour-icon' ><img className='fabric-icon' src={FabricIcon} alt="" />Fabric</span>
                    <div className='flavour-wrapper'>
                        <img className='fabric-icon' src={FabricIcon} alt="" />
                        <span>Fabric</span>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default NewInstance;
