use rocket::Route;
use crate::routes::Routes;

mod user;

use user::UserRoutes;

pub(crate) struct ApiRoutes;

impl Routes for ApiRoutes {
    fn routes() -> Vec<Route> {
        [UserRoutes::routes()].concat()
    }
}
