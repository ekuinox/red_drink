import * as React from 'react'
import { makeStyles } from '@material-ui/core/styles'
import { AppBar, Toolbar, Typography, Button, Avatar, IconButton } from '@material-ui/core'
import { TokenResponse } from '../red_drink_apis/get_token'

const useStyles = makeStyles((theme) => ({
    root: { flexGrow: 1 },
    title: { flexGrow: 1 }
}))

const AvatarIcon = ({user}: { user: TokenResponse }) => {
    if (user == null) return <div></div>
    return (
        <Avatar alt={user.display_name} src={user.avatar_url} />
    )
}

export const Header = (props: { tokenResponse?: TokenResponse, title: string}) => {
    const classes = useStyles()
    
    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    <Typography className={classes.title}>{ props.title }</Typography>
                    { props.tokenResponse == null ? (
                        <Button variant="contained" color="secondary" href="/login">Login</Button>
                    ) : (
                        <AvatarIcon user={props.tokenResponse} />
                    )}
                </Toolbar>
            </AppBar>
        </div>
    )
}