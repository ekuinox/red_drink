use rocket::Route;
use crate::routes::Routes;

mod get_user;

use self::get_user::*;

pub(crate) struct UserRoutes;

impl Routes for UserRoutes {
    fn routes() -> Vec<Route> {
        routes![self::get, self::get_user_by_username]
    }
}
