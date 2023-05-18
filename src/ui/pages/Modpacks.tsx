import React, { useEffect, useState } from 'react';
import '../styles/Modpacks.css';
import { invoke } from '@tauri-apps/api';

interface ModrinthResponse {
    hits: Modpack[]
    limit: number
    offset: number
    total_hits: number
}

interface Modpack {
    project_id: string
    title: string
    icon_url: string
    featured_gallery: string
}

interface ModpackItemProps {
    modpack: Modpack
}

function Modpacks(): JSX.Element {
    const [modpacks, setModpacks] = useState<Modpack[]>([]);

    function getModpacks(): void {
        invoke('get_modrinth_modpacks').then(mp => {
            const newModpacks = (mp as ModrinthResponse).hits;
            setModpacks(newModpacks);
        }).catch(e => {});
    }

    useEffect(() => {
        getModpacks();
    }, []);

    return (
        <div className='modpacks'>
            <div className='page-info'>
                <span className='page-title'>Modpacks</span>
                <span>Ready-to-play modpacks</span>
            </div>
            <div className='modpacks-container'>
                {modpacks.map((element, key) => <ModpackItem key={key} modpack={element} />)}
            </div>
        </div>
    );
}

export default Modpacks;

function ModpackItem({ modpack }: ModpackItemProps): JSX.Element {
    return (
        <div className='modpack-item'>
            <img className='modpack-item-bg' src={modpack.featured_gallery} alt="" />
            <div className='modpack-item-bg-gradient'>
                <img className='modpack-item-icon' src={modpack.icon_url} alt="" />
            </div>
        </div>
    );
}
