import * as React from 'react'
import { getToken, GetTokenResponse } from './red_drink_apis/get_token'
import { Header } from './components/Header'
import { Body } from './components/Body'

export const App = () => {
    const [tokenResponse, setTokenResponse] = React.useState<GetTokenResponse>({})
    React.useEffect(() => {
        getToken().then(setTokenResponse)
    }, [ '' ])
    return (
        <>
            <Header token={tokenResponse.token} />
            <Body token={tokenResponse.token} />
        </>
    )
}