import { For, JSX, Show, createSignal } from 'solid-js';
import '../styles/NewInstance.scss';
import { flavours } from '../../data/constants';
import { Flavours, Pages } from '../../data/enums';
import TextInput from '../components/TextInput/TextInput';
import VanillaVersionMenu from '../components/version-menus/VanillaVersionMenu';
import BaseButton from '../components/BaseButton/BaseButton';
import { PageNavigationProps } from '../../data/models';

export interface VersionMenuProps {
    selectedVersion: string;
    setSelectedVersion: (version: string) => void;
}

function NewInstance(props: PageNavigationProps): JSX.Element {
    const [selectedFlavour, setSelectedFlavour] = createSignal<Flavours | null>(
        null,
    );

    const [instanceName, setInstanceName] = createSignal('');
    const [isInstanceNameValid, setIsInstanceNameValid] = createSignal(false);
    const [isTitleModified, setIsTitleModified] = createSignal(false);

    const [selectedVersion, setSelectedVersion] = createSignal('');

    let gridRef!: HTMLDivElement;

    const handleNameInputChange = (name: string): void => {
        setInstanceName(name);
        setIsInstanceNameValid(
            /^(?!^(?:(?:CON|PRN|AUX|NUL|COM\d|LPT\d)(?:\..*)?)$)[^<>:"/\\|?*\x00-\x1F]*[^<>:"/\\|?*\x00-\x1F .]$/i.test(
                name.trim(),
            ),
        );

        setIsTitleModified(name.length !== 0);
    };

    const handleVersionChange = (version: string): void => {
        setSelectedVersion(version);
        if (!isTitleModified()) {
            setInstanceName(version.substring(0, 32));
            setIsInstanceNameValid(true);
        }
    };

    const handleClickable = (): boolean => {
        return selectedVersion().length > 0 || isInstanceNameValid();
    };

    return (
        <div id='new-instance' ref={gridRef}>
            <For each={flavours}>
                {(flavour) => (
                    <div
                        class='flavour'
                        classList={{
                            selected: selectedFlavour() === flavour.flavour,
                        }}
                        onClick={() => {
                            setSelectedFlavour(flavour.flavour);
                            gridRef.classList.toggle('selected');
                        }}
                    >
                        <img src={flavour.background} />
                        <span class='flavour-icon'>
                            <flavour.icon />
                            {flavour.name}
                        </span>
                        <div class='flavour-wrapper'>
                            <div class='flavour-info'>
                                <flavour.icon />
                                <span>{flavour.name}</span>
                            </div>
                            <TextInput
                                value={instanceName()}
                                onInput={handleNameInputChange}
                                isValid={isInstanceNameValid()}
                            />
                            <Show when={flavour.flavour == Flavours.Vanilla}>
                                <VanillaVersionMenu
                                    setSelectedVersion={handleVersionChange}
                                    selectedVersion={selectedVersion()}
                                />
                            </Show>
                            <BaseButton
                                text='Create Instance'
                                clickable={handleClickable()}
                                onClick={() => {
                                    props.setCurrentPage(Pages.Library);
                                }}
                            />
                        </div>
                    </div>
                )}
            </For>
        </div>
    );
}

export default NewInstance;
