import { type JSX } from 'solid-js';
import './TextInput.scss';
import { TickIcon, WarnIcon } from '../../../assets/icons/Icons';

interface TextInputProps {
    value: string;
    onInput: (value: string) => void;
    isValid: boolean;
}

function TextInput(props: TextInputProps): JSX.Element {
    const handleChange = (event: string) => {
        props.onInput(event);
    };

    return (
        <div class='text-input'>
            <div class='input-wrapper'>
                <input
                    type='text'
                    maxLength={32}
                    title=''
                    required
                    value={props.value}
                    onInput={(event) => {
                        handleChange(event.target.value);
                    }}
                    spellcheck={false}
                />
                <span class='input-label'>Instance Name</span>
                <hr />
            </div>
            {props.isValid ? <TickIcon /> : <WarnIcon />}
        </div>
    );
}

export default TextInput;
