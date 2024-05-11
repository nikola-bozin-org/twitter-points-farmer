mod dbg;
mod tasks;
mod users;

use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    let mut router = Router::new();
    router = router
        .merge(dbg::routes())
        .merge(users::routes())
        .merge(tasks::routes());
    router
}
