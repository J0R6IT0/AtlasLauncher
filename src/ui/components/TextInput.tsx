import React from 'react';
import '../styles/TextInput.css';
import { AlertTriangleIcon, CheckIcon } from '../../assets/icons/Icons';

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
            {props.inputValid ? <CheckIcon /> : <AlertTriangleIcon />}
        </div>
    );
}

export default TextInput;
