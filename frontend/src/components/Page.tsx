import * as React from 'react'
import { Header } from './Header'
import { Body } from './Body'

export const Page = (props: {
    title: string
    token?: string
    children: React.ReactNode
}) => {
    return (
        <>
            <Header token={props.token} />
            <Body token={props.token}>{ props.children }</Body>
        </>
    )
}