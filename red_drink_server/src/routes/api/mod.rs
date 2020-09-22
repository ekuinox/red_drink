use rocket::Route;
use crate::routes::Routes;

mod action;
mod user;

use action::ActionRoutes;
use user::UserRoutes;

pub(crate) struct ApiRoutes;

impl Routes for ApiRoutes {
    fn routes() -> Vec<Route> {
        [
            ActionRoutes::routes(),
            UserRoutes::routes(),
        ].concat()
    }
}
