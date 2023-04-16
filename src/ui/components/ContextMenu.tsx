import React, { useEffect, useRef } from 'react';

import '../styles/ContextMenu.css';

interface ContextMenuProps {
    target: Element | null
    onClose: () => void
    position: { x: number, y: number }
}

function ContextMenu(props: ContextMenuProps): JSX.Element {
    const menuRef = useRef<HTMLDivElement>(null);

    const handleOutsideClick = (event: MouseEvent): void => {
        console.log(Math.random());
        const menu = document.querySelector('.context-menu') as HTMLElement;
        if (!menu.contains(event.target as Node)) {
            menuRef.current?.classList.remove('visible');
            setTimeout(() => {
                props.onClose();
            }, 300);
        }
    };

    useEffect(() => {
        if (menuRef.current == null) {
            return;
        }
        setTimeout(() => {
            menuRef.current?.classList.add('visible');
        }, 10);
        const rect = menuRef.current.getBoundingClientRect();
        const windowWidth = window.innerWidth;
        const windowHeight = window.innerHeight;

        if (rect.right > windowWidth) {
            const newLeft = props.position.x - rect.width;
            menuRef.current.style.left = `${newLeft}px`;
        }
        if (rect.bottom > windowHeight) {
            const newTop = props.position.y - rect.height;
            menuRef.current.style.top = `${newTop}px`;
        }

        document.addEventListener('click', handleOutsideClick);

        return () => {
            document.removeEventListener('click', handleOutsideClick);
        };
    }, [props.position.x, props.position.y]);

    return (
        <div ref={menuRef} className='context-menu' style={{ left: props.position.x, top: props.position.y }}>

        </div>
    );
}

export default ContextMenu;
