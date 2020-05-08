import * as React from 'react'
import { TokenResponse } from '../red_drink_apis/get_token'

export const Body = (props: { tokenResponse?: TokenResponse, children: React.ReactNode}) => {
    return <div>{ props.children }</div>
}