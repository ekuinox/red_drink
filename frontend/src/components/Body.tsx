import React from 'react';
import { User } from '../red_drink_apis/get_user';

export const Body = (props: {
  user?: User;
  children: React.ReactNode;
}): JSX.Element => {
  return <div>{props.children}</div>;
};
