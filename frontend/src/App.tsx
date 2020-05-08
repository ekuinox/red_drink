import * as React from 'react'
import { getToken, GetTokenResponse } from './red_drink_apis/get_token'
import { Page } from './components/Page'

export const App = () => {
    const [tokenResponse, setTokenResponse] = React.useState<GetTokenResponse>({})
    React.useEffect(() => {
        getToken().then(setTokenResponse)
    }, [ '' ])
    return (
        <>
            <Page title="Top" token={tokenResponse.token}>
                <p>{tokenResponse.token == null ? 'ログインしてね' : 'こんにちは'}</p>
            </Page>
        </>
    )
}