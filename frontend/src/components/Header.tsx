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
    menuButton: { marginRight: theme.spacing(2) },
    title: { flexGrow: 1 }
}))

const AvatarIcon = ({user}: { user?: GitHubUser }) => {
    if (user == null) return <div></div>
    return (
        <Avatar alt={user.name} src={user.avatar_url} />
    )
}

export const Header = ({token}: { token?: string}) => {
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
                    <IconButton edge="start" className={classes.menuButton} color="inherit" aria-label="menu">
                        <MenuIcon />
                    </IconButton>
                    <Typography className={classes.title}>Top</Typography>
                    <Button variant="contained" color="secondary" href={ token == null ? "/login" : "/logout" }>
                        { token == null ? "Login" : "Logout" }
                    </Button>
                    <AvatarIcon user={user} />
                </Toolbar>
            </AppBar>
        </div>
    )
}