import React from 'react';
import '../styles/BaseToggle.css';

interface BaseToggleProps {
    onEnable: () => void;
    onDisable: () => void;
    default: boolean;
}

function BaseToggle(props: BaseToggleProps): JSX.Element {
    return (
        <div className='base-toggle'>
            <label className='switch'>
                <input
                    defaultChecked={props.default}
                    type='checkbox'
                    onClick={(event) => {
                        if (event.currentTarget.checked) {
                            props.onEnable();
                        } else {
                            props.onDisable();
                        }
                    }}
                />
                <span className='slider'></span>
            </label>
        </div>
    );
}

export default BaseToggle;
