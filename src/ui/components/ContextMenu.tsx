import React, { useEffect, useRef } from 'react';

import '../styles/ContextMenu.css';

interface ContextMenuProps {
    target: Element | null
    onClose: () => void
    position: { x: number, y: number }
}

function ContextMenu(props: ContextMenuProps): JSX.Element {
    const menuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (menuRef.current == null) {
            return;
        }

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
    }, [props.position.x, props.position.y]);

    return (
        <div ref={menuRef} className='context-menu' style={{ left: props.position.x, top: props.position.y }}>

        </div>
    );
}

export default ContextMenu;
