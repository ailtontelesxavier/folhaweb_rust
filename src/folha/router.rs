use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{folha::view, state::SharedState};

pub fn router() -> Router<SharedState> {
    Router::new().merge(router_folha())
}

fn router_folha() -> Router<SharedState> {
    Router::new()
        .route("/folha", get(view::list_folha))
        .merge(api_folha_router())
}

fn api_folha_router() -> Router<SharedState> {
    Router::new().route("/imprimir-contra-cheque/{id}", get(view::folha_api_by_id))
}
