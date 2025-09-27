mod cadastro;
mod error;
mod filters;
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
    cadastro::router as router_cadastro,
    filters::register_filters,
    middlewares::handle_forbidden,
    state::{AppState, LoginPayload, MessageResponse, SharedState},
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
        templates
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

    let rotas_privadas = Router::new()
        .route("/home", get(index))
        .route("/logout", get(logout))
        .nest("/cadastro", router_cadastro())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::autenticar,
        ));

    let app = Router::new()
        .route("/", get(set_messages_handler))
        .route("/read-messages", get(read_messages_handler))
        .route("/login", get(get_login).post(login))
        .nest_service("/static", server_dir)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(handle_forbidden)) // Middleware para 403
        .merge(rotas_privadas)
        .fallback(page_not_found_handler)
        .layer(MessagesManagerLayer)
        .layer(session_layer)
        .with_state(state.clone());

    info!("Starting server on http://0.0.0.0:2000");
    debug!("Server running");
    //println!("Server running on http://0.0.0.0:2000");
    axum::serve(listener, app).await.unwrap();
}

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

pub async fn page_not_found_handler(
    State(state): State<SharedState>,
) -> Result<Html<String>, impl IntoResponse> {
    match state.templates.get_template("404.html") {
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

async fn get_login(
    State(state): State<SharedState>,
    Query(params): Query<HashMap<String, String>>,
    messages: Messages,
) -> Result<Html<String>, impl IntoResponse> {
    let messages_vec: Vec<_> = messages
        .clone()
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "level": m.level.to_string(),
                "text": m.to_string()
            })
        })
        .collect();

    let context = minijinja::context! {
        messages => messages_vec,
    };

    match state.templates.get_template("login.html") {
        Ok(template) => match template.render(context) {
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

async fn login(
    State(state): State<SharedState>,
    Form(payload): Form<LoginPayload>,
    messages: Messages,
) -> Response<Body> {
    match UserService::get_by_username(&state.db, &payload.username).await {
        Ok(user) => {
            if !user.is_active {
                messages.error(&format!("Incorrect username or password"));

                return Redirect::to("/login").into_response();
            }

            if let Ok(false) | Err(_) =
                UserService::verify_password(&payload.password, &user.password)
            {
                messages.error(&format!("Incorrect username or password"));
                return Redirect::to("/login").into_response();
            }

            /* if !UserService::is_valid_otp(&payload.client_secret, &user.otp_base32.clone().unwrap())
            {
                messages.error(&format!("Incorrect username or password"));
                return Redirect::to("/login").into_response();
            } */

            let access_token = middlewares::gerar_token(&user.username);

            // Busca os módulos usando o service
            let json_data: String = match sqlx::query_as!(Module, r#"SELECT * FROM module"#)
                .fetch_all(&*state.db)
                .await
            {
                Ok(paginated_result) => {
                    // Converte os módulos para JSON
                    let modules: Vec<Value> = paginated_result
                        .iter()
                        .map(|m| {
                            json!({
                                "id": m.id,
                                "title": m.title,
                            })
                        })
                        .collect();

                    json!(modules).to_string()
                }
                Err(err) => {
                    debug!("Erro ao buscar módulos: {}", err);
                    messages.error(&format!("Erro ao buscar módulos: {}", err));
                    json!([]).to_string() // Array vazio em caso de erro
                }
            };

            // Configura os cookies
            let access_token_expire_minutes = env::var("ACCESS_TOKEN_EXPIRE_MINUTES")
                .unwrap_or_else(|_| "3600".to_string())
                .parse::<i64>()
                .unwrap_or(3600);

            let max_age = Duration::minutes(access_token_expire_minutes);
            let expires = OffsetDateTime::now_utc() + Duration::hours(1);

            // Formata a data de expiração no formato RFC2822
            let expires_formatted = expires.format(&Rfc2822).unwrap();

            // Cria a resposta
            //let mut response = Response::new(Body::empty());
            let mut response = Redirect::to("/").into_response();

            // Adiciona os cookies
            let modules_cookie = format!(
                "modules={}; Max-Age={}; Path=/",
                percent_encoding::percent_encode(
                    json_data.as_bytes(),
                    percent_encoding::NON_ALPHANUMERIC
                ),
                max_age.whole_seconds()
            );

            // Adiciona os cookies ao cabeçalho da resposta
            response
                .headers_mut()
                .append(SET_COOKIE, HeaderValue::from_str(&modules_cookie).unwrap());

            // Cria o cookie de access_token
            let access_token_cookie = format!(
                "access_token={}; HttpOnly; SameSite=Strict; Max-Age={}; Path=/; Expires={}",
                percent_encode(access_token.as_bytes(), NON_ALPHANUMERIC),
                max_age.whole_seconds(),
                expires_formatted
            );

            response.headers_mut().append(
                SET_COOKIE,
                HeaderValue::from_str(&access_token_cookie).unwrap(),
            );

            response
        }
        Err(err) => {
            messages.error(&format!("Senha não atualizada: {}", err));
            Redirect::to("/login").into_response()
        }
    }
}

async fn logout() -> impl IntoResponse {
    // Cria uma resposta de sucesso
    /* let mut response = Response::builder()
    .status(StatusCode::OK)
    .body(Body::empty())
    .unwrap(); */
    let mut response = Redirect::to("/login").into_response();

    // Invalida o cookie de access_token definindo uma data no passado
    let expired_cookie = format!(
        "access_token=; HttpOnly; SameSite=Strict; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Max-Age=0"
    );

    response
        .headers_mut()
        .append(SET_COOKIE, HeaderValue::from_str(&expired_cookie).unwrap());

    // Se você tiver outros cookies para limpar, adicione aqui
    // Exemplo para limpar o cookie 'usuario':
    let expired_usuario_cookie =
        "usuario=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Max-Age=0";
    response.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(expired_usuario_cookie).unwrap(),
    );

    response
}
