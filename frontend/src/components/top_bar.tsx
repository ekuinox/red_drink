import * as React from 'react'
import { makeStyles } from '@material-ui/core/styles'
import AppBar from '@material-ui/core/AppBar'
import Toolbar from '@material-ui/core/Toolbar'
import Button from '@material-ui/core/Button'
import { Link } from '@material-ui/core'

interface GitHubUser {
    name: string
    avatar_url: string
    login: string
}

const useStyles = makeStyles((theme) => ({
    root: { flexGrow: 1 },
    menuButton: { marginRight: theme.spacing(2) },
    title: { flexGrow: 1 }
}));

export const TopBar = ({token}: { token?: string}) => {
    const classes = useStyles()
    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    <Button variant="contained" color="secondary" href={ token == null ? "/login" : "/logout" }>
                        { token == null ? "Login" : "Logout" }
                    </Button>
                </Toolbar>
            </AppBar>
        </div>
    )
}