use crate::db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}
