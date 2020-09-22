mod invoke;

use rocket::Route;
use crate::routes::Routes;
use invoke::*;

pub(crate) struct ActionRoutes;

impl Routes for ActionRoutes {
    fn routes() -> Vec<Route> {
        routes![invoke]
    }
}
