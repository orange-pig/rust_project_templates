use db::DbState;

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
}
