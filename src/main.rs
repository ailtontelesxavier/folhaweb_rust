mod error;
mod filters;
mod folha;
mod middlewares;
mod repository;
mod state;

use std::{collections::HashMap, env, sync::Arc};

use axum::{
    Form, Router,
    body::Body,
    extract::{Query, State},
    http::{
        HeaderValue, Method, Response, StatusCode,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, SET_COOKIE},
    },
    middleware::{self},
    response::{Html, IntoResponse, Redirect},
    routing::get,
};

use minijinja::{Environment, context, path_loader};
use percent_encoding::{NON_ALPHANUMERIC, percent_encode};
use serde::Deserialize;
use serde_json::{Value, json};
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc2822};
use tokio;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::{debug, info};
use tracing_subscriber::{fmt::format, layer::SubscriberExt, util::SubscriberInitExt};

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use axum_messages::{Messages, MessagesManagerLayer};

use crate::{
    filters::register_filters,
    folha::router,
    middlewares::handle_forbidden,
    state::{AppState, MessageResponse, SharedState},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Carrega os templates
    // Crie o ambiente MiniJinja
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    // Registre os filtros
    register_filters(&mut env);

    let templates = Arc::new(env);

    let state = Arc::new(AppState {
        db: Arc::new(db_pool),
        templates,
        message: Arc::new(MessageResponse {
            status: "info".to_string(),
            message: "Sistema iniciado".to_string(),
        }),
    });

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            "info,debug,tower_http=debug",
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2000").await.unwrap();

    let server_dir = ServeDir::new("static");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let app = Router::new()
        /* .route("/", get(rota_index))
        .route("/index", get(rota_index2)) */
        .route("/", get(set_messages_handler))
        .route("/read-messages", get(read_messages_handler))
        .nest("/folha", router())
        .nest_service("/static", server_dir)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(handle_forbidden)) // Middleware para 403
        .layer(MessagesManagerLayer)
        .layer(session_layer)
        .with_state(state.clone());

    info!("Starting server on http://0.0.0.0:2000");
    debug!("Server running");
    //println!("Server running on http://0.0.0.0:2000");
    axum::serve(listener, app).await.unwrap();
}

/* async fn rota_index(mut session: WritableSession) -> impl IntoResponse {
    // Exemplo: contador de visitas
    let visits: i32 = session.get("visits").unwrap_or(0);
    session.insert("visits", visits + 1).unwrap();

    //Redirect::to("/index")

    Html(format!("Você já visitou esta página {} vezes.", visits))
}

async fn rota_index2(mut session: WritableSession) -> impl IntoResponse {
    let flash: Option<String> = session.get("flash");

    let menssage = if let Some(msg) = flash {
        session.remove("flash");
        msg
    } else {
        "Sem mensagens".to_string()
    };

    // devolve um response válido (texto simples)
    (StatusCode::OK, menssage)
}

async fn lista(mut session: WritableSession) -> impl IntoResponse {
    session.insert("flash", "Item carregado salvo com sucesso!").unwrap();
    Redirect::to("/folha")
}
 */

async fn set_messages_handler(messages: Messages) -> impl IntoResponse {
    messages.success("✅ Mensagem de sucesso adicionada!");
    Redirect::to("/folha/folha")
}

async fn read_messages_handler(messages: Messages) -> impl IntoResponse {
    let messages_vec: Vec<_> = messages
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "level": m.level.to_string(),
                "text": m.to_string()
            })
        })
        .collect();

    format!("Mensagens: {:?}", messages_vec)
}

async fn index2(
    State(state): State<SharedState>,
    messages: Messages,
) -> Result<Html<String>, impl IntoResponse> {
    // Coletar mensagens
    let messages_vec: Vec<_> = messages
        .into_iter()
        .map(|m| {
            // estrutura simples para enviar ao template
            serde_json::json!({
                "level": m.level.to_string(),
                "text": m.to_string()
            })
        })
        .collect();

    // Renderizar template com mensagens
    match state.templates.get_template("folha_list.html") {
        Ok(template) => match template.render(context! { messages => messages_vec }) {
            Ok(html) => Ok(Html(html)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao renderizar template: {}", err),
            )
                .into_response()),
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao carregar template: {}", err),
        )
            .into_response()),
    }
}

async fn index(State(state): State<SharedState>) -> Result<Html<String>, impl IntoResponse> {
    match state.templates.get_template("principal.html") {
        Ok(template) => match template.render({}) {
            Ok(html) => Ok(Html(html)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao renderizar template: {}", err),
            )
                .into_response()),
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao carregar template: {}", err),
        )
            .into_response()),
    }
}
