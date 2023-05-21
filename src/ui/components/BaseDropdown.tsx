import React, { useEffect, useRef, useState } from 'react';
import '../styles/BaseDropdown.css';
import ArrowIcon from '../../assets/icons/arrow-right.svg';

interface BaseDropdownProps {
    placeholder: string;
    values: string[];
    default?: string;
    autoScroll: boolean;
    onSelect: (value: string) => void;
}

function BaseDropdown(props: BaseDropdownProps): JSX.Element {
    const [active, setActive] = useState(false);
    const [selectedValue, setSelectedValue] = useState(
        props.default !== undefined ? props.default : ''
    );

    const toggleActive = (): void => {
        setActive(!active);
    };

    const menuRef = useRef<HTMLDivElement>(null);

    const handleOutsideClick = (event: MouseEvent): void => {
        const menu = menuRef.current;
        if (menu !== null && !menu.contains(event.target as Node)) {
            setActive(false);
        }
    };

    useEffect(() => {
        document.addEventListener('click', handleOutsideClick);
        return () => {
            document.removeEventListener('click', handleOutsideClick);
        };
    }, []);

    return (
        <div
            ref={menuRef}
            className={`base-dropdown ${active ? 'active' : ''} ${
                selectedValue.length > 0 ? 'selected' : ''
            }`}
            onClick={toggleActive}
        >
            <span className='dropdown-label'>{props.placeholder}</span>
            <span className='selected-value'>{selectedValue}</span>
            <img src={ArrowIcon} />
            <div className='dropdown-values'>
                {props.values.map((value, key) => (
                    <div key={key}>
                        <div
                            className={`dropdown-value clickable ${
                                selectedValue === value ? 'selected' : ''
                            }`}
                            onClick={() => {
                                setSelectedValue(value);
                                props.onSelect(value);
                            }}
                        >
                            <span>
                                {selectedValue === value && (
                                    <div className='dot'></div>
                                )}
                                {value}
                            </span>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default BaseDropdown;
