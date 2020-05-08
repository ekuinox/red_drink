import * as React from 'react'

export const Body = (props: { token?: string, children: React.ReactNode}) => {
    return <div>{ props.children }</div>
}