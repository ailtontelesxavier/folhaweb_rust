use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;

use crate::kanban::{model::*, repository::KanbanRepository};

pub struct KanbanService;

impl KanbanService {
    // ==== BOARD SERVICES ====

    pub async fn create_board(
        db: &Arc<PgPool>,
        user_id: i32,
        request: CreateBoardRequest,
    ) -> Result<KanbanBoard> {
        // Validações
        if request.title.trim().is_empty() {
            return Err(anyhow::anyhow!("O título do quadro é obrigatório"));
        }

        if request.title.len() > 255 {
            return Err(anyhow::anyhow!(
                "O título do quadro deve ter no máximo 255 caracteres"
            ));
        }

        // Validar cor se fornecida
        if let Some(ref color) = request.color {
            if !Self::is_valid_hex_color(color) {
                return Err(anyhow::anyhow!(
                    "Cor inválida. Use formato hexadecimal (#RRGGBB)"
                ));
            }
        }

        let board = KanbanRepository::create_board(db, user_id, &request).await?;

        // Criar colunas padrão para o novo board
        let default_columns = vec![
            CreateColumnRequest {
                title: "A Fazer".to_string(),
                color: Some("#EF4444".to_string()),
                max_cards: None,
            },
            CreateColumnRequest {
                title: "Em Progresso".to_string(),
                color: Some("#F59E0B".to_string()),
                max_cards: None,
            },
            CreateColumnRequest {
                title: "Revisão".to_string(),
                color: Some("#8B5CF6".to_string()),
                max_cards: None,
            },
            CreateColumnRequest {
                title: "Concluído".to_string(),
                color: Some("#10B981".to_string()),
                max_cards: None,
            },
        ];

        for column_request in default_columns {
            KanbanRepository::create_column(db, board.id, &column_request).await?;
        }

        Ok(board)
    }

    pub async fn get_user_boards(db: &Arc<PgPool>, user_id: i32) -> Result<Vec<KanbanBoard>> {
        KanbanRepository::get_boards_by_user(db, user_id).await
    }

    pub async fn get_board_by_id(
        db: &Arc<PgPool>,
        board_id: i32,
        user_id: i32,
    ) -> Result<Option<KanbanBoard>> {
        KanbanRepository::get_board_by_id(db, board_id, user_id).await
    }

    pub async fn update_board(
        db: &Arc<PgPool>,
        board_id: i32,
        user_id: i32,
        request: UpdateBoardRequest,
    ) -> Result<Option<KanbanBoard>> {
        // Validações
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(anyhow::anyhow!("O título do quadro é obrigatório"));
            }
            if title.len() > 255 {
                return Err(anyhow::anyhow!(
                    "O título do quadro deve ter no máximo 255 caracteres"
                ));
            }
        }

        if let Some(ref color) = request.color {
            if !Self::is_valid_hex_color(color) {
                return Err(anyhow::anyhow!(
                    "Cor inválida. Use formato hexadecimal (#RRGGBB)"
                ));
            }
        }

        KanbanRepository::update_board(db, board_id, user_id, &request).await
    }

    pub async fn delete_board(db: &Arc<PgPool>, board_id: i32, user_id: i32) -> Result<bool> {
        KanbanRepository::delete_board(db, board_id, user_id).await
    }

    // ==== COLUMN SERVICES ====

    pub async fn create_column(
        db: &Arc<PgPool>,
        board_id: i32,
        user_id: i32,
        request: CreateColumnRequest,
    ) -> Result<KanbanColumn> {
        // Verificar se o board pertence ao usuário
        match Self::get_board_by_id(db, board_id, user_id).await? {
            Some(_) => {}
            None => return Err(anyhow::anyhow!("Quadro não encontrado ou acesso negado")),
        }

        // Validações
        if request.title.trim().is_empty() {
            return Err(anyhow::anyhow!("O título da coluna é obrigatório"));
        }

        if request.title.len() > 255 {
            return Err(anyhow::anyhow!(
                "O título da coluna deve ter no máximo 255 caracteres"
            ));
        }

        if let Some(ref color) = request.color {
            if !Self::is_valid_hex_color(color) {
                return Err(anyhow::anyhow!(
                    "Cor inválida. Use formato hexadecimal (#RRGGBB)"
                ));
            }
        }

        if let Some(max_cards) = request.max_cards {
            if max_cards < 1 {
                return Err(anyhow::anyhow!("O limite de cards deve ser pelo menos 1"));
            }
        }

        KanbanRepository::create_column(db, board_id, &request).await
    }

    pub async fn update_column(
        db: &Arc<PgPool>,
        column_id: i32,
        user_id: i32,
        request: UpdateColumnRequest,
    ) -> Result<Option<KanbanColumn>> {
        // TODO: Verificar se a coluna pertence a um board do usuário

        // Validações
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(anyhow::anyhow!("O título da coluna é obrigatório"));
            }
            if title.len() > 255 {
                return Err(anyhow::anyhow!(
                    "O título da coluna deve ter no máximo 255 caracteres"
                ));
            }
        }

        if let Some(ref color) = request.color {
            if !Self::is_valid_hex_color(color) {
                return Err(anyhow::anyhow!(
                    "Cor inválida. Use formato hexadecimal (#RRGGBB)"
                ));
            }
        }

        if let Some(max_cards) = request.max_cards {
            if max_cards < 1 {
                return Err(anyhow::anyhow!("O limite de cards deve ser pelo menos 1"));
            }
        }

        KanbanRepository::update_column(db, column_id, &request).await
    }

    pub async fn delete_column(db: &Arc<PgPool>, column_id: i32, user_id: i32) -> Result<bool> {
        // TODO: Verificar se a coluna pertence a um board do usuário
        KanbanRepository::delete_column(db, column_id).await
    }

    // ==== CARD SERVICES ====

    pub async fn create_card(
        db: &Arc<PgPool>,
        column_id: i32,
        user_id: i32,
        request: CreateCardRequest,
    ) -> Result<KanbanCard> {
        // TODO: Verificar se a coluna pertence a um board do usuário

        // Validações
        if request.title.trim().is_empty() {
            return Err(anyhow::anyhow!("O título do card é obrigatório"));
        }

        if request.title.len() > 255 {
            return Err(anyhow::anyhow!(
                "O título do card deve ter no máximo 255 caracteres"
            ));
        }

        if let Some(ref priority) = request.priority {
            if !["low", "medium", "high", "urgent"].contains(&priority.as_str()) {
                return Err(anyhow::anyhow!(
                    "Prioridade inválida. Use: low, medium, high ou urgent"
                ));
            }
        }

        if let Some(ref color) = request.color {
            if !Self::is_valid_hex_color(color) {
                return Err(anyhow::anyhow!(
                    "Cor inválida. Use formato hexadecimal (#RRGGBB)"
                ));
            }
        }

        KanbanRepository::create_card(db, column_id, &request).await
    }

    pub async fn get_card_by_id(db: &Arc<PgPool>, card_id: i32) -> Result<Option<KanbanCard>> {
        KanbanRepository::get_card_by_id(db, card_id).await
    }

    pub async fn update_card(
        db: &Arc<PgPool>,
        card_id: i32,
        user_id: i32,
        request: UpdateCardRequest,
    ) -> Result<Option<KanbanCard>> {
        // TODO: Verificar se o card pertence a um board do usuário

        // Validações
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(anyhow::anyhow!("O título do card é obrigatório"));
            }
            if title.len() > 255 {
                return Err(anyhow::anyhow!(
                    "O título do card deve ter no máximo 255 caracteres"
                ));
            }
        }

        if let Some(ref priority) = request.priority {
            if !["low", "medium", "high", "urgent"].contains(&priority.as_str()) {
                return Err(anyhow::anyhow!(
                    "Prioridade inválida. Use: low, medium, high ou urgent"
                ));
            }
        }

        if let Some(ref color) = request.color {
            if !Self::is_valid_hex_color(color) {
                return Err(anyhow::anyhow!(
                    "Cor inválida. Use formato hexadecimal (#RRGGBB)"
                ));
            }
        }

        KanbanRepository::update_card(db, card_id, &request).await
    }

    pub async fn move_card(
        db: &Arc<PgPool>,
        card_id: i32,
        user_id: i32,
        request: MoveCardRequest,
    ) -> Result<Option<KanbanCard>> {
        // TODO: Verificar se o card pertence a um board do usuário
        KanbanRepository::move_card(db, card_id, &request).await
    }

    pub async fn delete_card(db: &Arc<PgPool>, card_id: i32, user_id: i32) -> Result<bool> {
        // TODO: Verificar se o card pertence a um board do usuário
        KanbanRepository::delete_card(db, card_id).await
    }

    // ==== COMMENT SERVICES ====

    pub async fn create_comment(
        db: &Arc<PgPool>,
        card_id: i32,
        user_id: i32,
        request: CreateCommentRequest,
    ) -> Result<KanbanComment> {
        // TODO: Verificar se o card pertence a um board do usuário

        // Validações
        if request.content.trim().is_empty() {
            return Err(anyhow::anyhow!("O conteúdo do comentário é obrigatório"));
        }

        if request.content.len() > 2000 {
            return Err(anyhow::anyhow!(
                "O comentário deve ter no máximo 2000 caracteres"
            ));
        }

        KanbanRepository::create_comment(db, card_id, user_id, &request).await
    }

    // ==== COMPLEX SERVICES ====

    pub async fn get_board_with_data(
        db: &Arc<PgPool>,
        board_id: i32,
        user_id: i32,
    ) -> Result<Option<BoardWithColumnsAndCards>> {
        KanbanRepository::get_board_with_columns_and_cards(db, board_id, user_id).await
    }

    pub async fn get_card_with_comments(
        db: &Arc<PgPool>,
        card_id: i32,
        user_id: i32,
    ) -> Result<Option<CardWithComments>> {
        // TODO: Verificar se o card pertence a um board do usuário
        KanbanRepository::get_card_with_comments(db, card_id).await
    }

    // ==== UTILITY FUNCTIONS ====

    fn is_valid_hex_color(color: &str) -> bool {
        if !color.starts_with('#') || color.len() != 7 {
            return false;
        }

        color[1..].chars().all(|c| c.is_ascii_hexdigit())
    }
}
