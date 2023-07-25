import './BaseButton.scss';
import { type JSX } from 'solid-js';

interface BaseButtonProps {
    clickable: boolean;
    onClick: () => void;
    text: string;
}

function BaseButton(props: BaseButtonProps): JSX.Element {
    const handleClick = () => {
        if (props.clickable) props.onClick();
    };

    return (
        <div
            class='base-button clickable'
            classList={{ clickable: props.clickable }}
            onClick={handleClick}
        >
            <span>{props.text}</span>
        </div>
    );
}

export default BaseButton;
