import MinecraftForge from '../assets/images/minecraft-forge.webp';
import MinecraftFabric from '../assets/images/minecraft-fabric.webp';
import MinecraftQuilt from '../assets/images/minecraft-quilt.webp';
import MinecraftCover from '../assets/images/minecraft-cover.webp';

import {
    GridIcon,
    HomeIcon,
    SearchIcon,
    PlusIcon,
    SettingsIcon,
    BoxIcon,
    QuiltIcon,
} from '../assets/icons/Icons';
import { Flavours, Pages } from './enums';

export const pages = [
    {
        page: Pages.Home,
        icon: HomeIcon,
        name: 'Home',
    },
    {
        page: Pages.Library,
        icon: GridIcon,
        name: 'Library',
    },
    {
        page: Pages.New,
        icon: PlusIcon,
        name: 'New Instance',
    },
    {
        page: Pages.Modpacks,
        icon: SearchIcon,
        name: 'Modpacks',
    },
    {
        page: Pages.Settings,
        icon: SettingsIcon,
        name: 'Settings',
    },
];

export const flavours = [
    {
        flavour: Flavours.Vanilla,
        name: 'Vanilla',
        background: MinecraftCover,
        icon: BoxIcon,
    },
    {
        flavour: Flavours.Forge,
        name: 'Forge',
        background: MinecraftForge,
        icon: BoxIcon,
    },
    {
        flavour: Flavours.Fabric,
        name: 'Fabric',
        background: MinecraftFabric,
        icon: BoxIcon,
    },
    {
        flavour: Flavours.Quilt,
        name: 'Quilt',
        background: MinecraftQuilt,
        icon: QuiltIcon,
    },
];

export const vanillaVersionTypes = [
    {
        type: 'release',
        name: 'Release',
    },
    {
        type: 'snapshot',
        name: 'Snapshot',
    },
    {
        type: 'old_beta',
        name: 'Beta',
    },
    {
        type: 'old_alpha',
        name: 'Alpha',
    },
];
