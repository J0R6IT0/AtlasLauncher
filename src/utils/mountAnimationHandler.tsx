import { useEffect } from 'react';

const mountAnimationHandler = (
    ref: React.RefObject<HTMLElement>,
    onClose: () => void
): void => {
    const handleOutsideClick = (event: MouseEvent): void => {
        if (
            ref.current !== null &&
            ref.current !== undefined &&
            !ref.current.contains(event.target as Node)
        ) {
            onClose();
        }
    };

    useEffect(() => {
        setTimeout(() => {
            ref.current?.classList.add('visible');
            document.addEventListener('click', handleOutsideClick);
        }, 10);
        return () => {
            document.removeEventListener('click', handleOutsideClick);
        };
    }, []);
};

export default mountAnimationHandler;
