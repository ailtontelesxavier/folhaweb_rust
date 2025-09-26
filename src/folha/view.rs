use std::collections::HashMap;
use std::str::FromStr;

use axum_messages::Messages;

use anyhow::Result;
use axum::Json;
use axum::{
    Extension, Form,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use bigdecimal::BigDecimal;
use minijinja::context;

use serde_json::Value;
use tracing::debug;

use crate::folha::service::FolhaService;
use crate::repository::ListParams;
use crate::state::SharedState;

/* pub async fn salvar_item(mut session: WritableSession) -> impl IntoResponse {
    session.insert("flash", "Item salvo com sucesso!").unwrap();
    Redirect::to("/lista")
}

pub async fn listar(session: ReadableSession, State(state): State<AppState>) -> impl IntoResponse {
    let flash: Option<String> = session.get("flash");
    if flash.is_some() {
        session.remove("flash");
    }

    state.render("lista.html", context! { flash => flash })
}
 */

pub async fn list_folha(
    State(state): State<SharedState>,
    Query(params): Query<ListParams>,
    messages: Messages,
) -> impl IntoResponse {
    let service = FolhaService::new();

    // Coletar mensagens do axum_messages
    let messages_vec: Vec<_> = messages
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "level": m.level.to_string(),
                "text": m.to_string()
            })
        })
        .collect();

    // Usar o PermissionService para buscar dados paginados
    let result = service
        .get_paginated(
            &state.db,
            params.find.as_deref(),
            params.page.unwrap_or(1),
            params.page_size.unwrap_or(10),
        )
        .await;

    match result {
        Ok(paginated_response) => {
            let context = minijinja::context! {
                rows => paginated_response.data,
                current_page => paginated_response.page,
                total_pages => paginated_response.total_pages,
                page_size => paginated_response.page_size,
                total_records => paginated_response.total_records,
                find => params.find.unwrap_or_default(),
                messages => messages_vec,
            };

            match state.templates.get_template("folha_list.html") {
                Ok(template) => match template.render(context) {
                    Ok(html) => Html(html).into_response(),
                    Err(err) => {
                        debug!("Erro ao renderizar template: {}", err);
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                },
                Err(err) => {
                    debug!("Erro ao carregar template: {}", err);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(err) => {
            debug!("Erro ao buscar linhas: {}", err);
            Redirect::to(&"/").into_response()
        }
    }
}
