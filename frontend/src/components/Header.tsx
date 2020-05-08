import * as React from 'react'
import { makeStyles } from '@material-ui/core/styles'
import { AppBar, Toolbar, Typography, Button, Avatar, IconButton } from '@material-ui/core'
import MenuIcon from '@material-ui/icons/Menu'

interface GitHubUser {
    name: string
    avatar_url: string
    login: string
}

const useStyles = makeStyles((theme) => ({
    root: { flexGrow: 1 },
    title: { flexGrow: 1 }
}))

const AvatarIcon = ({user}: { user?: GitHubUser }) => {
    if (user == null) return <div></div>
    return (
        <Avatar alt={user.name} src={user.avatar_url} />
    )
}

export const Header = (props: { token?: string, title: string}) => {
    const classes = useStyles()
    const [user, setUser] = React.useState<GitHubUser | null>()
    React.useEffect(() => {
        if (props.token != null) {
            fetch('https://api.github.com/user', {
                method: 'GET',
                headers: { Authorization: `token ${props.token}` }
            }).then(r => r.json()).then(setUser)
        }
    }, [ props.token ])
    
    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    <Typography className={classes.title}>{ props.title }</Typography>
                    { props.token == null ? (
                        <Button variant="contained" color="secondary" href="/login">Login</Button>
                    ) : (
                        <AvatarIcon user={user} />
                    )}
                </Toolbar>
            </AppBar>
        </div>
    )
}