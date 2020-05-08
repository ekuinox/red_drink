import * as React from 'react'

export const Body = (props: { token?: string }) => {
    if (props.token == null) {
        return <div>ログインしてね</div>
    }
    return <div>こんにちは</div>
}