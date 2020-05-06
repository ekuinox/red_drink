import * as React from 'react'

interface GitHubUser {
    name: string
    avatar_url: string
    login: string
}

const UserProfile = ({profile}: { profile: GitHubUser}) => (
    <div>
        <h2>{ profile.name }</h2>
        <img src={ profile.avatar_url }></img>
    </div>
)

export const TopBar = ({token}: { token?: string}) => {
    if (token == null) {
        return (
            <div>
                <a href="/request_token">ログインはこちら</a>
            </div>
        )
    }

    const [user, setUser] = React.useState<GitHubUser | null>()
    React.useEffect(() => {
        fetch('https://api.github.com/user', {
            method: 'GET',
            headers: { Authorization: `token ${token}` }
        }).then(response => response.json()).then(setUser)
    }, [ '' ])
    
    return user == null ? <div>loading...</div> : <UserProfile profile={user} />
}