use serde::{Serialize, Deserialize};
use ts_rs::TS;
use utoipa::ToSchema;

pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[derive(Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = ".generated/HSLink.ts")]
pub struct HSLink {
    pub name: String,
    pub value: String,
}