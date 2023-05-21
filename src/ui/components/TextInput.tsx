import React from 'react';
import '../styles/TextInput.css';
import CheckIcon from '../../assets/icons/check.svg';
import AlertIcon from '../../assets/icons/alert-triangle.svg';

interface TextInputProps {
    value: string;
    onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
    name: string;
    inputValid: boolean;
}

function TextInput(props: TextInputProps): JSX.Element {
    return (
        <div className='input'>
            <input
                type='text'
                value={props.value}
                required
                spellCheck='false'
                onChange={props.onChange}
                maxLength={32}
                title=''
            />
            <span className='floating-input-label'>{props.name}</span>
            <img
                className='input-image'
                src={props.inputValid ? CheckIcon : AlertIcon}
                alt=''
            />
        </div>
    );
}

export default TextInput;
