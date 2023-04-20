import React from 'react';
import '../styles/TextButton.css';

interface TextButtonProps {
    text: string
    clickable: boolean
    onClick: () => void
}

function TextButton(props: TextButtonProps): JSX.Element {
    return (
        <div className={`text-button clickable ${props.clickable ? 'valid' : ''}`} onClick={() => {
            if (props.clickable) {
                props.onClick();
            }
        }}><span>{props.text}</span></div>
    );
}

export default TextButton;
