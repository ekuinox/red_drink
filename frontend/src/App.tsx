import React, { useState, useEffect } from 'react';
import { getUser, User } from './red_drink_apis/get_user';
import { Page } from './components/Page';

export const App = (): JSX.Element => {
  const [user, setUser] = useState<User>();
  useEffect(() => {
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
