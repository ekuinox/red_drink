import * as React from 'react'
import { makeStyles } from '@material-ui/core/styles'
import { AppBar, Toolbar, Typography, Button, Avatar, Menu, MenuItem, ButtonBase, Link } from '@material-ui/core'
import { TokenResponse } from '../red_drink_apis/get_token'

const NoneTextDecorationLink = (props: { to: string, children: React.ReactNode }) => {
    return <Link href={ props.to } style={{textDecoration: 'none'}}>{ props.children }</Link>
}

const useStyles = makeStyles((theme) => ({
    root: { flexGrow: 1 },
    title: { flexGrow: 1 }
}))

const HeaderMenu = ({user}: { user: TokenResponse }) => {
    const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);

    const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    }

    const handleClose = () => {
        setAnchorEl(null);
    };

    return (
        <>
            <ButtonBase onClick={handleClick}>
                <Avatar alt={user.display_name} src={user.avatar_url} />
            </ButtonBase>
            <Menu
                id='header-menu'
                anchorEl={anchorEl}
                keepMounted
                open={Boolean(anchorEl)}
                onClose={handleClose}
                style={{
                    marginTop: '5vh'
                }}
            >
                <MenuItem onClick={handleClose}><NoneTextDecorationLink to='/logout'>Logout</NoneTextDecorationLink></MenuItem>
            </Menu>
        </>
    )
}

export const Header = (props: { tokenResponse?: TokenResponse, title: string}) => {
    const classes = useStyles()
    
    return (
        <div className={classes.root}>
            <AppBar position='static'>
                <Toolbar>
                    <Typography className={classes.title}>{ props.title }</Typography>
                    { props.tokenResponse == null ? (
                        <Button variant='contained' color='secondary' href='/login'>Login</Button>
                    ) : (
                        <HeaderMenu user={props.tokenResponse} />
                    )}
                </Toolbar>
            </AppBar>
        </div>
    )
}