use axum::{
    Form,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};
use axum_messages::Messages;
use minijinja::context;
use serde_json::json;
use validator::Validate;

use crate::{
    kanban::{model::*, schema::*, service::KanbanService},
    state::SharedState,
};

// ==== BOARD HANDLERS ====

pub async fn board_list_page(
    State(state): State<SharedState>,
    messages: Messages,
) -> Result<Html<String>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    match KanbanService::get_user_boards(&state.db, user_id).await {
        Ok(boards) => {
            let messages_vec: Vec<_> = messages
                .into_iter()
                .map(|m| {
                    json!({
                        "level": m.level.to_string(),
                        "text": m.to_string()
                    })
                })
                .collect();

            let context = context! {
                boards => boards,
                messages => messages_vec,
                user_id => user_id,
            };

            match state.templates.get_template("kanban/board_list.html") {
                Ok(template) => match template.render(context) {
                    Ok(html) => Ok(Html(html)),
                    Err(err) => Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Erro ao renderizar template: {}", err),
                    )),
                },
                Err(err) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Erro ao carregar template: {}", err),
                )),
            }
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao carregar quadros: {}", err),
        )),
    }
}

pub async fn board_view_page(
    State(state): State<SharedState>,
    Path(board_id): Path<i32>,
    messages: Messages,
) -> Result<Html<String>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    match KanbanService::get_board_with_data(&state.db, board_id, user_id).await {
        Ok(Some(board_data)) => {
            let messages_vec: Vec<_> = messages
                .into_iter()
                .map(|m| {
                    json!({
                        "level": m.level.to_string(),
                        "text": m.to_string()
                    })
                })
                .collect();

            let context = context! {
                board_data => board_data,
                messages => messages_vec,
                user_id => user_id,
            };

            match state.templates.get_template("kanban/board_view.html") {
                Ok(template) => match template.render(context) {
                    Ok(html) => Ok(Html(html)),
                    Err(err) => Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Erro ao renderizar template: {}", err),
                    )),
                },
                Err(err) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Erro ao carregar template: {}", err),
                )),
            }
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "Quadro não encontrado".to_string())),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao carregar quadro: {}", err),
        )),
    }
}

pub async fn create_board_form(
    State(state): State<SharedState>,
    messages: Messages,
) -> Result<Html<String>, impl IntoResponse> {
    let messages_vec: Vec<_> = messages
        .into_iter()
        .map(|m| {
            json!({
                "level": m.level.to_string(),
                "text": m.to_string()
            })
        })
        .collect();

    let context = context! {
        messages => messages_vec,
    };

    match state.templates.get_template("kanban/board_form.html") {
        Ok(template) => match template.render(context) {
            Ok(html) => Ok(Html(html)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao renderizar template: {}", err),
            )),
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao carregar template: {}", err),
        )),
    }
}

pub async fn create_board_post(
    State(state): State<SharedState>,
    messages: Messages,
    Form(form): Form<CreateBoardSchema>,
) -> impl IntoResponse {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    // Validar dados
    if let Err(errors) = form.validate() {
        let mut error_messages = Vec::new();
        for (field, field_errors) in errors.field_errors() {
            for error in field_errors {
                let error_msg = error
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Erro no campo {}", field));
                error_messages.push(error_msg);
            }
        }
        for msg in error_messages {
            messages.clone().error(&msg);
        }
        return axum::response::Redirect::to("/kanban/boards/new").into_response();
    }

    let request = CreateBoardRequest {
        title: form.title,
        description: form.description,
        color: form.color,
    };

    match KanbanService::create_board(&state.db, user_id, request).await {
        Ok(board) => {
            messages.success(&format!("Quadro '{}' criado com sucesso!", board.title));
            axum::response::Redirect::to(&format!("/kanban/boards/{}", board.id)).into_response()
        }
        Err(err) => {
            messages.error(&format!("Erro ao criar quadro: {}", err));
            axum::response::Redirect::to("/kanban/boards/new").into_response()
        }
    }
}

// ==== API HANDLERS ====

pub async fn api_get_boards(
    State(state): State<SharedState>,
) -> Result<Json<ApiResponse<Vec<KanbanBoard>>>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    match KanbanService::get_user_boards(&state.db, user_id).await {
        Ok(boards) => Ok(Json(ApiResponse::success(boards))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<KanbanBoard>>::error(format!(
                "Erro ao carregar quadros: {}",
                err
            ))),
        )),
    }
}

pub async fn api_get_board(
    State(state): State<SharedState>,
    Path(board_id): Path<i32>,
) -> Result<Json<ApiResponse<BoardWithColumnsAndCards>>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    match KanbanService::get_board_with_data(&state.db, board_id, user_id).await {
        Ok(Some(board_data)) => Ok(Json(ApiResponse::success(board_data))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<BoardWithColumnsAndCards>::error(
                "Quadro não encontrado".to_string(),
            )),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!(
                "Erro ao carregar quadro: {}",
                err
            ))),
        )),
    }
}

pub async fn api_create_board(
    State(state): State<SharedState>,
    Json(form): Json<CreateBoardSchema>,
) -> Result<Json<ApiResponse<KanbanBoard>>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    // Validar dados
    if let Err(errors) = form.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, field_errors)| {
                field_errors.iter().map(|error| {
                    error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Erro no campo {}", field))
                })
            })
            .collect();

        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(error_messages.join(", "))),
        ));
    }

    let request = CreateBoardRequest {
        title: form.title,
        description: form.description,
        color: form.color,
    };

    match KanbanService::create_board(&state.db, user_id, request).await {
        Ok(board) => Ok(Json(ApiResponse::success_with_message(
            board,
            "Quadro criado com sucesso!".to_string(),
        ))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Erro ao criar quadro: {}", err))),
        )),
    }
}

pub async fn api_create_column(
    State(state): State<SharedState>,
    Path(board_id): Path<i32>,
    Json(form): Json<CreateColumnSchema>,
) -> Result<Json<ApiResponse<KanbanColumn>>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    // Validar dados
    if let Err(errors) = form.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, field_errors)| {
                field_errors.iter().map(|error| {
                    error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Erro no campo {}", field))
                })
            })
            .collect();

        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(error_messages.join(", "))),
        ));
    }

    let request = CreateColumnRequest {
        title: form.title,
        color: form.color,
        max_cards: form.max_cards,
    };

    match KanbanService::create_column(&state.db, board_id, user_id, request).await {
        Ok(column) => Ok(Json(ApiResponse::success_with_message(
            column,
            "Coluna criada com sucesso!".to_string(),
        ))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Erro ao criar coluna: {}", err))),
        )),
    }
}

pub async fn api_create_card(
    State(state): State<SharedState>,
    Path(column_id): Path<i32>,
    Json(form): Json<CreateCardSchema>,
) -> Result<Json<ApiResponse<KanbanCard>>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    // Validar dados
    if let Err(errors) = form.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, field_errors)| {
                field_errors.iter().map(|error| {
                    error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Erro no campo {}", field))
                })
            })
            .collect();

        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(error_messages.join(", "))),
        ));
    }

    let request = CreateCardRequest {
        title: form.title,
        description: form.description,
        priority: form.priority,
        tags: form.tags,
        color: form.color,
        due_date: form.due_date,
    };

    match KanbanService::create_card(&state.db, column_id, user_id, request).await {
        Ok(card) => Ok(Json(ApiResponse::success_with_message(
            card,
            "Card criado com sucesso!".to_string(),
        ))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Erro ao criar card: {}", err))),
        )),
    }
}

pub async fn api_move_card(
    State(state): State<SharedState>,
    Path(card_id): Path<i32>,
    Json(form): Json<MoveCardSchema>,
) -> Result<Json<ApiResponse<KanbanCard>>, impl IntoResponse> {
    // TODO: Obter user_id da sessão
    let user_id = 1; // Temporário

    let request = MoveCardRequest {
        column_id: form.column_id,
        position: form.position,
    };

    match KanbanService::move_card(&state.db, card_id, user_id, request).await {
        Ok(Some(card)) => Ok(Json(ApiResponse::success_with_message(
            card,
            "Card movido com sucesso!".to_string(),
        ))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Card não encontrado".to_string())),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Erro ao mover card: {}", err))),
        )),
    }
}
