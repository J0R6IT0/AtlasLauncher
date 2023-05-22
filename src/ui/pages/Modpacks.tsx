import React, { useEffect, useState } from 'react';
import '../styles/Modpacks.css';
import { invoke } from '@tauri-apps/api';

interface ModrinthResponse {
    hits: Modpack[];
    limit: number;
    offset: number;
    total_hits: number;
}

interface Modpack {
    project_id: string;
    title: string;
    icon_url: string;
    featured_gallery: string;
}

interface ModpackItemProps {
    modpack: Modpack;
}

function Modpacks(): JSX.Element {
    const [modpacks, setModpacks] = useState<Modpack[]>([]);

    function getModpacks(): void {
        invoke('get_modrinth_modpacks')
            .then((mp) => {
                const newModpacks = (mp as ModrinthResponse).hits;
                setModpacks(newModpacks);
            })
            .catch((e) => {});
    }

    useEffect(() => {
        getModpacks();
    }, []);

    return (
        <div className='modpacks'>
            <div className='modpacks-container'>
                <div className='grid'>
                    {modpacks.map((element, key) => (
                        <ModpackItem key={key} modpack={element} />
                    ))}
                </div>
            </div>
        </div>
    );
}

export default Modpacks;

function ModpackItem({ modpack }: ModpackItemProps): JSX.Element {
    return (
        <div className='modpack'>
            <img
                className='modpack-bg'
                src={
                    modpack.featured_gallery !== undefined &&
                    modpack.featured_gallery !== null &&
                    modpack.featured_gallery.length > 0
                        ? modpack.featured_gallery
                        : modpack.icon_url
                }
                alt=''
            />
            <div className='modpack-bg-gradient'>
                <img className='modpack-icon' src={modpack.icon_url} alt='' />
                <div className='modpack-info'>
                    <span className='modpack-title'>{modpack.title}</span>
                </div>
            </div>
        </div>
    );
}
