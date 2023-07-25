import { createSignal, type JSX, createResource } from 'solid-js';
import SideBar from './ui/components/SideBar/SideBar';
import TitleBar from './ui/components/TitleBar/TitleBar';
import './ui/styles/App.scss';
import { EventStatus, Pages } from './data/enums';
import { Route, Routes, useNavigate } from '@solidjs/router';
import NewInstance from './ui/pages/NewInstance';
import Library from './ui/pages/Library';
import { Motion, Presence } from '@motionone/solid';
import { Rerun } from '@solid-primitives/keyed';
import AccountMenu from './ui/components/AccountsMenu/AccountsMenu';
import { invoke } from '@tauri-apps/api/tauri';
import { MSAuthEvent, type MinecraftAccount } from './data/models';
import { listen } from '@tauri-apps/api/event';

function App(): JSX.Element {
    // accounts

    const getAccounts = async () =>
        (await invoke('get_accounts')) as MinecraftAccount[];

    const [accounts, { refetch }] =
        createResource<MinecraftAccount[]>(getAccounts);
    const [accountsMenuVisible, setAccountsMenuVisible] = createSignal(false);

    const toggleAccountsMenuVisibility = (visible?: boolean): void => {
        if (typeof visible !== 'undefined') setAccountsMenuVisible(visible);
        else setAccountsMenuVisible(!accountsMenuVisible());
    };

    listen('msauth', (event: MSAuthEvent) => {
        if (event.payload.base.status === EventStatus.Success) {
            refetch();
        }
    });

    // navigation

    const [currentPage, setCurrentPage] = createSignal(Pages.Library);

    const navigate = useNavigate();

    const handlePageChange = (page: Pages): void => {
        if (currentPage() !== page) {
            setCurrentPage(page);
            navigate('/' + page);
        }
    };

    const handleNavigation = (): void => {
        const newRoute = window.location.pathname;
        handlePageChange(parseInt(newRoute.replace('/', '')));
    };

    window.addEventListener('popstate', handleNavigation);

    return (
        <div id='app'>
            <TitleBar
                currentPage={currentPage()}
                setCurrentPage={handlePageChange}
            />
            <SideBar
                currentPage={currentPage()}
                setCurrentPage={handlePageChange}
                setAccountsMenuVisible={toggleAccountsMenuVisibility}
                activeAccount={
                    accounts()?.filter((account) => account.active)[0] || null
                }
            />
            <div id='content'>
                <Presence exitBeforeEnter>
                    <Rerun on={currentPage()}>
                        <Motion.div
                            initial={{ opacity: 0, x: 50 }}
                            animate={{
                                opacity: 1,
                                x: 0,
                                transition: { delay: 0.05 },
                            }}
                            transition={{
                                duration: 0.1,
                                easing: 'ease-in-out',
                            }}
                            exit={{ opacity: 0, x: -50 }}
                            id='page'
                        >
                            <Routes>
                                <Route
                                    path={['/', `/${Pages.Library}`]}
                                    component={Library}
                                />
                                <Route
                                    path={`/${Pages.New}`}
                                    element={
                                        <NewInstance
                                            currentPage={currentPage()}
                                            setCurrentPage={handlePageChange}
                                        />
                                    }
                                />
                            </Routes>
                        </Motion.div>
                    </Rerun>
                </Presence>
                <AccountMenu
                    accounts={accounts() || []}
                    visible={accountsMenuVisible()}
                    setVisible={toggleAccountsMenuVisibility}
                    refreshAccounts={refetch}
                />
            </div>
        </div>
    );
}

export default App;
