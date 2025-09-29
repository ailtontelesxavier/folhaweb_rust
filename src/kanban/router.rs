use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{
    kanban::view::*,
    state::SharedState,
};

pub fn router() -> Router<SharedState> {
    Router::new()
        // Rotas das páginas web
        .route("/boards", get(board_list_page))
        .route("/boards/new", get(create_board_form).post(create_board_post))
        .route("/boards/:id", get(board_view_page))
        
        // API Routes
        .route("/api/boards", get(api_get_boards).post(api_create_board))
        .route("/api/boards/:id", get(api_get_board))
        .route("/api/boards/:id/columns", post(api_create_column))
        .route("/api/columns/:id/cards", post(api_create_card))
        .route("/api/cards/:id/move", put(api_move_card))
}