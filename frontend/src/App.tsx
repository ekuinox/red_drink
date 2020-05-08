import * as React from 'react'
import { getToken, TokenResponse } from './red_drink_apis/get_token'
import { Page } from './components/Page'

export const App = () => {
    const [tokenResponse, setTokenResponse] = React.useState<TokenResponse>()
    React.useEffect(() => {
        getToken().then(data => {
            if (data != null) {
                setTokenResponse(data)
            }
        })
    }, [ '' ])
    return (
        <>
            <Page title="Top" tokenResponse={tokenResponse}>
                <p>{tokenResponse == null ? 'ログインしてね' : 'こんにちは'}</p>
            </Page>
        </>
    )
}