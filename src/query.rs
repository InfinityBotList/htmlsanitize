use std::sync::Arc;

use axum::{response::{IntoResponse, Response}, http::StatusCode, extract::State, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{types::{AppState, HSLink}, sanitizer};

pub enum ServerError {
    Error(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::Error(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }
}

pub enum ServerResponse {
    Response(String),
}

impl IntoResponse for ServerResponse {
    fn into_response(self) -> Response {
        match self {
            ServerResponse::Response(e) => (StatusCode::OK, e).into_response(),
        }
    }
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = ".generated/Query.ts")]
pub enum Query {
    /// Sanitize a raw unparsed MD/HTML string
    SanitizeRaw {
        body: String,
    },
    /// Sanitize a raw unparsed MD/HTML string with extra links
    SanitizeTemplate {
        body: String,
        extra_links: Vec<HSLink>,
    },
    /// Sanitize the long description of a bot
    BotLongDescription {
        bot_id: String,
    },
    BlogPost {
        slug: String,
    }
}

#[debug_handler]
pub async fn query(
    State(app_state): State<Arc<AppState>>,
    Json(query): Json<Query>,
) -> Result<ServerResponse, ServerError> {
    match query {
        Query::SanitizeRaw { body } => {
            let sanitized = sanitizer::sanitize(&body);

            Ok(ServerResponse::Response(sanitized))
        },
        Query::SanitizeTemplate { body, extra_links } => {
            let sanitized = sanitizer::template(&body, extra_links);

            Ok(ServerResponse::Response(sanitized))
        },
        Query::BotLongDescription { bot_id } => {
            let row = sqlx::query!(
                "SELECT long, extra_links FROM bots WHERE bot_id = $1",
                bot_id
            )
            .fetch_optional(&app_state.pool)
            .await
            .map_err(|e| ServerError::Error(e.to_string()))?;

            match row {
                Some(bot) => {
                    // Deserialize the extra links
                    let extra_links: Vec<HSLink> = serde_json::from_value(bot.extra_links).map_err(|e| ServerError::Error(e.to_string()))?;

                    Ok(ServerResponse::Response(
                        sanitizer::template(
                            &bot.long,
                            extra_links
                        )
                    ))
                },
                None => Err(ServerError::Error("Bot not found".to_string()))
            }
        },
        Query::BlogPost { slug } => {
            let row = sqlx::query!(
                "SELECT content FROM blogs WHERE slug = $1",
                slug
            )
            .fetch_optional(&app_state.pool)
            .await
            .map_err(|e| ServerError::Error(e.to_string()))?;

            match row {
                Some(post) => {
                    Ok(ServerResponse::Response(
                        sanitizer::sanitize(
                            &post.content,
                        )
                    ))
                },
                None => Err(ServerError::Error("Blog post not found".to_string()))
            }
        },
    }
}