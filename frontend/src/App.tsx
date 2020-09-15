import React, { useState } from 'react';
import { getUser, User } from './red_drink_apis/get_user';
import { Page } from './components/Page';
import { useDidMound } from './common/hooks';

export const App = (): JSX.Element => {
  const [user, setUser] = useState<User>();
  useDidMound(() => getUser().then(setUser));

  return (
    <>
      <Page title="Top" user={user}>
        <p>{user == null ? 'ログインしてね' : 'こんにちは'}</p>
      </Page>
    </>
  );
};
