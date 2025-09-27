use minijinja::Environment;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
    pub templates: Arc<Environment<'static>>,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
    /* client_secret: String, */
}

pub type SharedState = Arc<AppState>;
