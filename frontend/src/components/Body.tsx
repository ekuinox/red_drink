import * as React from 'react';
import { User } from '../red_drink_apis/get_user';

export const Body = (props: { user?: User; children: React.ReactNode}) => {
    return <div>{ props.children }</div>
};
