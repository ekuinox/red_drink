import * as React from 'react'
import { getToken, GetTokenResponse } from './red_drink_apis/get_token'
import { TopBar } from './components/top_bar'

export const App = () => {
    const [tokenResponse, setTokenResponse] = React.useState<GetTokenResponse>({})
    React.useEffect(() => {
        getToken().then(setTokenResponse)
    }, [ '' ])
    return <TopBar token={tokenResponse.token} />
}