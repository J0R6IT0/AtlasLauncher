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
                    <div className='image-wrapper'>
                        <img className='flavour-icon' src={BoxIcon} alt="" />
                    </div>
                    <span>Vanilla</span>
                </div>
                <div className='flavour'>
                    <img className='flavour-background' src={MinecraftForge} alt="" />
                    <div className='image-wrapper'>
                        <img className='flavour-icon' src={ForgeIcon} alt="" />
                    </div>
                    <span>Forge</span>
                </div>
                <div className='flavour'>
                    <img className='flavour-background' src="https://zonacraft.net/wp-content/uploads/2022/07/Eden-Ring-Mod.png" alt="" />
                    <div className='image-wrapper'>
                        <img className='flavour-icon fabric-icon' src={FabricIcon} />
                    </div>
                    <span>Fabric</span>
                </div>

            </div>
        </div>
    );
}

export default NewInstance;
