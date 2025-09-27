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

use crate::cadastro::model::{Folha, Uf};
use crate::cadastro::schema::{CreateUf, UpdateUf};
use crate::cadastro::service::{FolhaService, UfService};
use crate::middlewares::CurrentUser;
use crate::repository::{ListParams, PaginatedResponse, PaginationQuery};
use crate::state::SharedState;

const PATH: &str = "cadastro";

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

            match state
                .templates
                .get_template(&format!("{}/folha_list.html", PATH))
            {
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
            Redirect::to(&format!("/{}/folha", PATH)).into_response()
        }
    }
}

/*
api regiao

*/
pub async fn folha_api_by_id(
    Path(id): Path<i64>,
    State(state): State<SharedState>,
) -> Result<Json<Folha>, StatusCode> {
    let service = FolhaService::new();
    let res = service.get_by_id(&state.db, id).await.map_err(|err| {
        debug!("error:{}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(res))
}

/*
==========================================
            Uf
==========================================
*/

pub async fn list_uf(
    State(state): State<SharedState>,
    Query(params): Query<ListParams>,
    messages: Messages,
) -> impl IntoResponse {
    let service = UfService::new();

    // Coletar mensagens do axum_messages
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
                messages => messages_vec
            };

            match state
                .templates
                .get_template(&format!("{}/uf_list.html", PATH))
            {
                Ok(template) => match template.render(context) {
                    Ok(html) => Html(html).into_response(),
                    Err(err) => {
                        debug!("Erro ao renderizar template: {}", err);
                        messages.error(format!("Erro ao renderizar template: {}", err));
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                },
                Err(err) => {
                    debug!("Erro ao carregar template: {}", err);
                    messages.error(format!("Erro ao carregar template: {}", err));
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(err) => {
            debug!("Erro ao buscar uf: {}", err);
            messages.error(format!("Erro ao carregar uf: {}", err));
            return Redirect::to(&format!("/{}/uf", PATH)).into_response();
        }
    }
}

pub async fn uf_form(
    State(state): State<SharedState>,
    Query(params): Query<HashMap<String, String>>,
    messages: Messages,
) -> Result<Html<String>, impl IntoResponse> {
    // Coletar mensagens do axum_messages
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
        messages => messages_vec
    };

    match state
        .templates
        .get_template(&format!("{}/uf_form.html", PATH))
    {
        Ok(template) => match template.render(context) {
            Ok(html) => Ok(Html(html)),
            Err(err) => {
                messages.error(format!("Erro ao renderizar template: {}", err));
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Erro ao renderizar template: {}", err),
                )
                    .into_response())
            }
        },
        Err(err) => {
            messages.error(format!("Erro ao carregar template: {}", err));
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao carregar template: {}", err),
            )
                .into_response())
        }
    }
}

pub async fn create_uf(
    State(state): State<SharedState>,
    Extension(current_user): Extension<CurrentUser>,
    messages: Messages,
    Form(body): Form<CreateUf>,
) -> Response {
    let service = UfService::new();

    match service.create(&*state.db, body).await {
        Ok(uf) => {
            messages.success("UF criado com sucesso!");
            Redirect::to(&format!("/{}/uf-form/{}", PATH, uf.id)).into_response()
        }
        Err(err) => {
            messages.error(format!("Erro ao criar UF: {}", err));
            Redirect::to(&format!("/{}/uf-form", PATH)).into_response()
        }
    }
}

pub async fn get_uf(
    State(state): State<SharedState>,
    messages: Messages,
    Path(id): Path<i32>,
    Query(params): Query<HashMap<String, String>>,
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
    let service = UfService::new();

    // Carregar o template
    let template = match state
        .templates
        .get_template(&format!("{}/uf_form.html", PATH))
    {
        Ok(t) => t,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao carregar template: {}", err),
            )
                .into_response());
        }
    };

    let uf = match service.get_by_id(&state.db, id).await {
        Ok(p) => p,
        Err(e) => {
            debug!("Erro ao buscar uf: {}", e);
            messages.error(&format!("Erro ao atualizar uf: {}", e));
            return Err(Redirect::to(&format!("/{}/uf-form", PATH)).into_response());
        }
    };

    // Preparar o contexto
    let ctx = context! {
        row => uf,
        messages => messages_vec,
    };

    match template.render(&ctx) {
        Ok(html) => Ok(Html(html)),
        Err(err) => {
            messages.error(format!("Falha ao renderizar template: {}", err));
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao renderizar template: {}", err),
            )
                .into_response())
        }
    }
}

pub async fn update_uf(
    State(state): State<SharedState>,
    Extension(current_user): Extension<CurrentUser>,
    messages: Messages,
    Path(id): Path<i32>,
    Form(input): Form<UpdateUf>,
) -> Response {
    let service = UfService::new();

    match service.update(&*state.db, id, input).await {
        Ok(_) => {
            messages.success("UF atualizado com sucesso!");
            Redirect::to(&format!("/{}/uf-form/{}", PATH, id)).into_response()
        }
        Err(err) => {
            messages.error(&format!("Erro ao atualizar uf: {}", err));
            Redirect::to(&format!("/{}/uf-form/{}", PATH, id)).into_response()
        }
    }
}

pub async fn delete_uf(
    State(state): State<SharedState>,
    Extension(current_user): Extension<CurrentUser>,
    messages: Messages,
    Path(id): Path<i32>,
) -> Response {
    let service = UfService::new();

    match service.delete(&*state.db, id).await {
        Ok(()) => {
            messages.success("UF excluído com sucesso!");
            Redirect::to(&format!("/{}/uf", PATH)).into_response()
        }
        Err(err) => {
            messages.error(&format!("Erro ao excluir UF: {}", err));
            Redirect::to(&format!("/{}/uf", PATH)).into_response()
        }
    }
}

pub async fn uf_api(
    Query(q): Query<PaginationQuery>,
    State(state): State<SharedState>,
) -> Result<Json<PaginatedResponse<Uf>>, StatusCode> {
    let service = UfService::new();
    let res = service
        .get_paginated(
            &state.db,
            q.find.as_deref(),
            q.page.unwrap_or(1) as i32,
            q.page_size.unwrap_or(10) as i32,
        )
        .await
        .map_err(|err| {
            debug!("error:{}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(res))
}
