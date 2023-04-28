import React, { useState } from 'react';
import '../styles/NewInstance.css';
import MinecraftForge from '../../assets/images/minecraft-forge.webp';
import MinecraftFabric from '../../assets/images/minecraft-fabric.webp';
import MinecraftCover from '../../assets/images/minecraft-cover.webp';
import CreateInstance from '../components/CreateInstance';
import BoxIcon from '../../assets/icons/box-thin.svg';
import ForgeIcon from '../../assets/icons/forge.svg';
import FabricIcon from '../../assets/icons/fabric.svg';

enum Flavours {
    Vanilla,
    Forge,
    Fabric
}

const flavours = [
    { id: Flavours.Vanilla, name: 'Vanilla', background: MinecraftCover, icon: BoxIcon },
    { id: Flavours.Forge, name: 'Forge', background: MinecraftForge, icon: ForgeIcon },
    { id: Flavours.Fabric, name: 'Fabric', background: MinecraftFabric, icon: FabricIcon }
];

interface NewInstanceProps {
    goToLibrary: () => void
}

const NewInstance = React.memo((props: NewInstanceProps) => {
    const [selectedFlavour, setSelectedFlavour] = useState<number | null>(null);
    return (
        <div className='new-instance'>
            <div className='page-info'>
                <span className='page-title'>New Instance</span>
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
                        <div className='flavour-content'>
                            {selectedFlavour === element.id && <CreateInstance flavour={selectedFlavour} goToLibrary={props.goToLibrary}/>}
                        </div>
                    </div>
                </div>)}
            </div>
        </div>
    );
});

NewInstance.displayName = 'NewInstance';

export default NewInstance;
