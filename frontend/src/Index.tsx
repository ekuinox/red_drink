import * as React from 'react'
import { render } from 'react-dom'
import * as qs from 'qs'
import { App } from './App'

interface Window_ extends Window {
    token?: string
}

declare const window: Window_

if (window.token == null) {
    const token: string | undefined = qs.parse(window.location.search.replace(/^\?/, '')).token
    if (token == null) {
        window.location.replace('/request_token')
    } else {
        window.token = token
    }
}

render(
    <App token={window.token} />,
    document.getElementById('root')
)