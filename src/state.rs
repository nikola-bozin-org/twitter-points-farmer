use crate::db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub dev_secret: String,
    pub security_hash:String
}
