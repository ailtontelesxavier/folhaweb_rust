use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{cadastro::view, state::SharedState};

pub fn router() -> Router<SharedState> {
    Router::new().merge(router_folha()).merge(uf_router())
}

fn router_folha() -> Router<SharedState> {
    Router::new()
        .route("/folha", get(view::list_folha))
        .merge(api_folha_router())
}

fn api_folha_router() -> Router<SharedState> {
    Router::new().route("/imprimir-contra-cheque/{id}", get(view::folha_api_by_id))
}

fn uf_router() -> Router<SharedState> {
    Router::new()
        .route("/uf", get(view::list_uf))
        .route("/uf-form", get(view::uf_form).post(view::create_uf))
        .route("/uf-form/{id}", get(view::get_uf).post(view::update_uf))
        .route("/uf/{id}", delete(view::delete_uf))
        .merge(api_uf_router())
}

fn api_uf_router() -> Router<SharedState> {
    Router::new().route("/uf-api", get(view::uf_api))
}
