import { onCleanup } from 'solid-js';

export default function clickOutside(
    el: Element,
    signal: (visible?: boolean) => void
) {
    const onClick = (event: MouseEvent) => {
        if (!el.contains(event.target as Node)) {
            setTimeout(() => signal(false), 5);
        }
    };
    document.body.addEventListener('click', onClick);

    onCleanup(() => document.body.removeEventListener('click', onClick));
}
