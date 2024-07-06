use sqlx::PgPool;

use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: PgPool,
    pub settings: Settings,
}
