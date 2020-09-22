use rocket::Route;
use crate::routes::Routes;

mod get_user;
mod roles;

use self::get_user::*;
use self::roles::*;

pub(crate) struct UserRoutes;

impl Routes for UserRoutes {
    fn routes() -> Vec<Route> {
        routes![get, get_user_by_username, get_roles]
    }
}
