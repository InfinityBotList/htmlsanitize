use serde::{Serialize, Deserialize};
use ts_rs::TS;

pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = ".generated/HSLink.ts")]
pub struct HSLink {
    pub name: String,
    pub value: String,
}