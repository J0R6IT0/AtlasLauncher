import React, { useEffect, useRef } from 'react';
import '../styles/BaseModal.css';
import TextButton from './TextButton';
import { AlertTriangleIcon } from '../../assets/icons/Icons';

interface BaseModalProps {
    description: string;
    title: string;
    onAccept: () => void;
    onClose: () => void;
}

function BaseModal(props: BaseModalProps): JSX.Element {
    const menuRef = useRef<HTMLDivElement>(null);

    const closeMenu = (): void => {
        menuRef.current?.classList.remove('visible');
        setTimeout(() => {
            props.onClose();
        }, 200);
    };

    const handleOutsideClick = (event: MouseEvent): void => {
        const menu = document.querySelector('.base-modal') as HTMLElement;
        if (!menu.contains(event.target as Node)) {
            closeMenu();
        }
    };

    useEffect(() => {
        setTimeout(() => {
            menuRef.current?.classList.add('visible');
            document.addEventListener('click', handleOutsideClick);
        }, 10);
        return () => {
            document.removeEventListener('click', handleOutsideClick);
        };
    }, []);

    return (
        <div ref={menuRef} className='base-modal'>
            <span className='base-modal-title'>
                <AlertTriangleIcon />
                {props.title}
            </span>
            <span className='base-modal-description'>{props.description}</span>
            <TextButton
                text='Accept'
                clickable={true}
                onClick={props.onAccept}
            />
        </div>
    );
}

export default BaseModal;
