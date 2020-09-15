import * as React from 'react';
import { getUser, User } from './red_drink_apis/get_user';
import { Page } from './components/Page';

export const App = () => {
    const [user, setUser] = React.useState<User>();
    React.useEffect(() => {
        getUser().then(setUser);
    }, []);

    return (
        <>
            <Page title="Top" user={user}>
                <p>{user == null ? 'ログインしてね' : 'こんにちは'}</p>
            </Page>
        </>
    );
};
