import * as React from 'react'

export const App = (props: { token: string }) => {
    interface UserProfile {
        username?: string
        profileImageUrl?: string
    }

    const [userProfile, setUserProfile] = React.useState<UserProfile>({})
    React.useEffect(() => {
        fetch('https://api.github.com/user', { headers: { Authorization: `token ${props.token}`}}).then(r => r.json()).then(data => {
            setUserProfile({
                username: data.login,
                profileImageUrl: data.avatar_url
            })
        })
    }, [''])

    return (
        <div>
            <p>{ userProfile.username }</p>
            <img src={userProfile.profileImageUrl}></img>
        </div>
    )
}