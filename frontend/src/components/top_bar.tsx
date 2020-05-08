import * as React from 'react'
import { makeStyles } from '@material-ui/core/styles'
import AppBar from '@material-ui/core/AppBar'
import Toolbar from '@material-ui/core/Toolbar'
import Button from '@material-ui/core/Button'
import Avatar from '@material-ui/core/Avatar'

interface GitHubUser {
    name: string
    avatar_url: string
    login: string
}

const useStyles = makeStyles((theme) => ({
    root: { flexGrow: 1 },
    menuButton: { marginRight: theme.spacing(2) },
    title: { flexGrow: 1 }
}))

const GitHubAvatar = ({user}: { user?: GitHubUser }) => {
    if (user == null) return <div></div>
    return (
        <div>
            { user.login }
            <Avatar alt={user.name} src={user.avatar_url} />
        </div>
    )
}

export const TopBar = ({token}: { token?: string}) => {
    const classes = useStyles()
    const [user, setUser] = React.useState<GitHubUser | null>()
    React.useEffect(() => {
        if (token != null) {
            fetch('https://api.github.com/user', {
                method: 'GET',
                headers: { Authorization: `token ${token}` }
            }).then(r => r.json()).then(setUser)
        }
    }, [ token ])
    
    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    <Button variant="contained" color="secondary" href={ token == null ? "/login" : "/logout" }>
                        { token == null ? "Login" : "Logout" }
                    </Button>
                    <GitHubAvatar user={user} />
                </Toolbar>
            </AppBar>
        </div>
    )
}