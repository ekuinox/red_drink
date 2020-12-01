use rocket::Route;
use crate::routes::Routes;

mod put_action;

use self::put_action::*;

pub(crate) struct ActionRoutes;

impl Routes for ActionRoutes {
    fn routes() -> Vec<Route> {
        routes![put_action]
    }
}
