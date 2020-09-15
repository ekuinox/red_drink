import React from 'react';
import { Header } from './Header';
import { Body } from './Body';
import { User } from '../red_drink_apis/get_user';

export const Page = (props: {
  title: string;
  user?: User;
  children: React.ReactNode;
}): JSX.Element => {
  return (
    <>
      <Header user={props.user} title={props.title} />
      <Body user={props.user}>{props.children}</Body>
    </>
  );
};
