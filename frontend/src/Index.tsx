import * as React from 'react'
import { render } from 'react-dom'
import * as qs from 'qs'
import { App } from './App'

fetch('/token').then(r => r.json()).then(data => {
    if (data == null || data.token == null) {
        window.location.replace('/request_token')
    } else {
        render(
            <App token={data.token} />,
            document.getElementById('root')
        )
    }
})