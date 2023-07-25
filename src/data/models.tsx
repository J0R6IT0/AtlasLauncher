import { EventStatus, Pages } from './enums';

export interface PageNavigationProps {
    currentPage: Pages;
    setCurrentPage: (page: Pages) => void;
}

export interface MinecraftAccount {
    username: string;
    uuid: string;
    active: boolean;
    avatar_64px: string;
}

export interface MSAuthEvent {
    payload: MSAuthEventPayload;
}

export interface MSAuthEventPayload {
    base: BaseEventPayload;
}

export interface BaseEventPayload {
    message: string;
    status: EventStatus;
}
