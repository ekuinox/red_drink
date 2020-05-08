import * as React from 'react'
import { Header } from './Header'
import { Body } from './Body'
import { TokenResponse } from '../red_drink_apis/get_token'

export const Page = (props: {
    title: string
    tokenResponse?: TokenResponse
    children: React.ReactNode
}) => {
    return (
        <>
            <Header tokenResponse={ props.tokenResponse } title={ props.title }/>
            <Body tokenResponse={props.tokenResponse}>{ props.children }</Body>
        </>
    )
}