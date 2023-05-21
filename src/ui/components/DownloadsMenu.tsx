import React, { useEffect, useRef, useState } from 'react';

import '../styles/DownloadsMenu.css';

interface DownloadBarProps {
    onClose: () => void;
    items: DownloadItemProps[];
}

export interface DownloadItemProps {
    name: string;
    total: number;
    downloaded: number;
    step: string;
}

function DownloadsBar(props: DownloadBarProps): JSX.Element {
    const menuRef = useRef<HTMLDivElement>(null);

    const handleOutsideClick = (event: MouseEvent): void => {
        const menu = menuRef.current;
        if (menu !== null && !menu.contains(event.target as Node)) {
            menu.classList.remove('visible');
            setTimeout(() => {
                props.onClose();
            }, 300);
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
        <div ref={menuRef} className='downloads-bar'>
            {props.items.length <= 0 && (
                <span className='no-active-dls'>No active downloads</span>
            )}
            {props.items.map((element, key) => (
                <DownloadItem
                    key={key}
                    name={element.name}
                    downloaded={element.downloaded / 2}
                    total={element.total}
                    step={element.step}
                />
            ))}
        </div>
    );
}

export default DownloadsBar;

function DownloadItem(props: DownloadItemProps): JSX.Element {
    const progressRef = useRef<HTMLDivElement>(null);
    const [isInfinite, setIsInfinite] = useState(false);

    useEffect(() => {
        if (progressRef.current !== null) {
            if (props.total > 0) {
                progressRef.current.style.width = `${
                    (props.downloaded / props.total) * 100
                }%`;
                setIsInfinite(false);
            } else {
                setIsInfinite(true);
            }
        }
    }, [props.downloaded, props.total]);

    return (
        <div className='download-item'>
            <div className='download-item-info'>
                <span className='download-item-name'>{props.name}</span>
                <span className='download-item-step'>{props.step}</span>
                <span className='download-item-progress'>
                    {(props.downloaded / 1_000_000).toFixed(2)} MB
                    {props.total > 0
                        ? ` / ${(props.total / 1_000_000.0).toFixed(2)} MB (${(
                              (props.downloaded / props.total) *
                              100
                          ).toFixed(0)}%)`
                        : ''}
                </span>
            </div>
            <div className='progress-bar'>
                <div className='bar'>
                    <div
                        ref={progressRef}
                        className={`progress ${isInfinite ? 'infinite' : ''}`}
                    />
                </div>
            </div>
        </div>
    );
}
