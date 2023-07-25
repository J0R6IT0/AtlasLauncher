import { Motion, Presence } from '@motionone/solid';
import './BaseMenu.scss';
import { For, Show, type JSX } from 'solid-js';
import clickOutside from '../../directives/click-outside';

interface BaseMenuProps {
    items: JSX.Element[];
    visible: boolean;
    setVisible: (visible?: boolean) => void;
}

function BaseMenu(props: BaseMenuProps): JSX.Element {
    return (
        <Presence exitBeforeEnter>
            <Show when={props.visible}>
                <Motion.div
                    initial={{ x: '-15rem' }}
                    animate={{ x: 0 }}
                    exit={{ x: '-15rem' }}
                    transition={{ duration: 0.15, easing: 'ease-in-out' }}
                    class='basemenu'
                    ref={(el) => {
                        clickOutside(el, props.setVisible);
                    }}
                >
                    <For each={props.items}>{(item) => item}</For>
                </Motion.div>
            </Show>
        </Presence>
    );
}

export default BaseMenu;
