use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

use super::model::*;

pub struct KanbanRepository;

impl KanbanRepository {
    // ==== BOARD OPERATIONS ====

    pub async fn create_board(
        db: &Arc<PgPool>,
        user_id: i32,
        request: &CreateBoardRequest,
    ) -> Result<KanbanBoard> {
        let board = sqlx::query_as!(
            KanbanBoard,
            r#"
            INSERT INTO kanban_board (user_id, title, description, color, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, title, description, color, is_active, position, created_at, updated_at
            "#,
            user_id,
            request.title,
            request.description,
            request.color,
            Utc::now()
        )
        .fetch_one(db.as_ref())
        .await?;

        Ok(board)
    }

    pub async fn get_boards_by_user(db: &Arc<PgPool>, user_id: i32) -> Result<Vec<KanbanBoard>> {
        let boards = sqlx::query_as!(
            KanbanBoard,
            r#"
            SELECT id, user_id, title, description, color, is_active, position, created_at, updated_at
            FROM kanban_board 
            WHERE user_id = $1 AND is_active = true
            ORDER BY position ASC, created_at ASC
            "#,
            user_id
        )
        .fetch_all(db.as_ref())
        .await?;

        Ok(boards)
    }

    pub async fn get_board_by_id(
        db: &Arc<PgPool>,
        board_id: i32,
        user_id: i32,
    ) -> Result<Option<KanbanBoard>> {
        let board = sqlx::query_as!(
            KanbanBoard,
            r#"
            SELECT id, user_id, title, description, color, is_active, position, created_at, updated_at
            FROM kanban_board 
            WHERE id = $1 AND user_id = $2 AND is_active = true
            "#,
            board_id,
            user_id
        )
        .fetch_optional(db.as_ref())
        .await?;

        Ok(board)
    }

    pub async fn update_board(
        db: &Arc<PgPool>,
        board_id: i32,
        user_id: i32,
        request: &UpdateBoardRequest,
    ) -> Result<Option<KanbanBoard>> {
        let board = sqlx::query_as!(
            KanbanBoard,
            r#"
            UPDATE kanban_board 
            SET title = COALESCE($3, title),
                description = COALESCE($4, description),
                color = COALESCE($5, color),
                position = COALESCE($6, position),
                is_active = COALESCE($7, is_active),
                updated_at = $8
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, title, description, color, is_active, position, created_at, updated_at
            "#,
            board_id,
            user_id,
            request.title,
            request.description,
            request.color,
            request.position,
            request.is_active,
            Utc::now()
        )
        .fetch_optional(db.as_ref())
        .await?;

        Ok(board)
    }

    pub async fn delete_board(db: &Arc<PgPool>, board_id: i32, user_id: i32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE kanban_board 
            SET is_active = false, updated_at = $3
            WHERE id = $1 AND user_id = $2
            "#,
            board_id,
            user_id,
            Utc::now()
        )
        .execute(db.as_ref())
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==== COLUMN OPERATIONS ====

    pub async fn create_column(
        db: &Arc<PgPool>,
        board_id: i32,
        request: &CreateColumnRequest,
    ) -> Result<KanbanColumn> {
        // Primeiro, obter a próxima posição
        let next_position: i32 = sqlx::query_scalar!(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM kanban_column WHERE board_id = $1",
            board_id
        )
        .fetch_one(db.as_ref())
        .await?
        .unwrap_or(0);

        let column = sqlx::query_as!(
            KanbanColumn,
            r#"
            INSERT INTO kanban_column (board_id, title, position, color, max_cards, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, board_id, title, position, color, max_cards, is_active, created_at, updated_at
            "#,
            board_id,
            request.title,
            next_position,
            request.color,
            request.max_cards,
            Utc::now()
        )
        .fetch_one(db.as_ref())
        .await?;

        Ok(column)
    }

    pub async fn get_columns_by_board(
        db: &Arc<PgPool>,
        board_id: i32,
    ) -> Result<Vec<KanbanColumn>> {
        let columns = sqlx::query_as!(
            KanbanColumn,
            r#"
            SELECT id, board_id, title, position, color, max_cards, is_active, created_at, updated_at
            FROM kanban_column 
            WHERE board_id = $1 AND is_active = true
            ORDER BY position ASC
            "#,
            board_id
        )
        .fetch_all(db.as_ref())
        .await?;

        Ok(columns)
    }

    pub async fn update_column(
        db: &Arc<PgPool>,
        column_id: i32,
        request: &UpdateColumnRequest,
    ) -> Result<Option<KanbanColumn>> {
        let column = sqlx::query_as!(
            KanbanColumn,
            r#"
            UPDATE kanban_column 
            SET title = COALESCE($2, title),
                position = COALESCE($3, position),
                color = COALESCE($4, color),
                max_cards = COALESCE($5, max_cards),
                is_active = COALESCE($6, is_active),
                updated_at = $7
            WHERE id = $1
            RETURNING id, board_id, title, position, color, max_cards, is_active, created_at, updated_at
            "#,
            column_id,
            request.title,
            request.position,
            request.color,
            request.max_cards,
            request.is_active,
            Utc::now()
        )
        .fetch_optional(db.as_ref())
        .await?;

        Ok(column)
    }

    pub async fn delete_column(db: &Arc<PgPool>, column_id: i32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE kanban_column 
            SET is_active = false, updated_at = $2
            WHERE id = $1
            "#,
            column_id,
            Utc::now()
        )
        .execute(db.as_ref())
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==== CARD OPERATIONS ====

    pub async fn create_card(
        db: &Arc<PgPool>,
        column_id: i32,
        request: &CreateCardRequest,
    ) -> Result<KanbanCard> {
        // Obter a próxima posição na coluna
        let next_position: i32 = sqlx::query_scalar!(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM kanban_card WHERE column_id = $1",
            column_id
        )
        .fetch_one(db.as_ref())
        .await?
        .unwrap_or(0);

        let card = sqlx::query_as!(
            KanbanCard,
            r#"
            INSERT INTO kanban_card (column_id, title, description, priority, tags, color, position, due_date, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, column_id, title, description, priority, tags, color, position, 
                      due_date, completed_at, is_archived, created_at, updated_at
            "#,
            column_id,
            request.title,
            request.description,
            request.priority.as_deref().unwrap_or("medium"),
            request.tags.as_deref(),
            request.color,
            next_position,
            request.due_date,
            Utc::now()
        )
        .fetch_one(db.as_ref())
        .await?;

        Ok(card)
    }

    pub async fn get_cards_by_column(db: &Arc<PgPool>, column_id: i32) -> Result<Vec<KanbanCard>> {
        let cards = sqlx::query_as!(
            KanbanCard,
            r#"
            SELECT id, column_id, title, description, priority, tags, color, position, 
                   due_date, completed_at, is_archived, created_at, updated_at
            FROM kanban_card 
            WHERE column_id = $1 AND is_archived = false
            ORDER BY position ASC
            "#,
            column_id
        )
        .fetch_all(db.as_ref())
        .await?;

        Ok(cards)
    }

    pub async fn get_card_by_id(db: &Arc<PgPool>, card_id: i32) -> Result<Option<KanbanCard>> {
        let card = sqlx::query_as!(
            KanbanCard,
            r#"
            SELECT id, column_id, title, description, priority, tags, color, position, 
                   due_date, completed_at, is_archived, created_at, updated_at
            FROM kanban_card 
            WHERE id = $1
            "#,
            card_id
        )
        .fetch_optional(db.as_ref())
        .await?;

        Ok(card)
    }

    pub async fn update_card(
        db: &Arc<PgPool>,
        card_id: i32,
        request: &UpdateCardRequest,
    ) -> Result<Option<KanbanCard>> {
        // Por enquanto, implementação simplificada - apenas para mover cards
        if let (Some(column_id), Some(position)) = (request.column_id, request.position) {
            return Self::move_card(db, card_id, &MoveCardRequest { column_id, position }).await;
        }
        
        // TODO: Implementar update completo depois
        Ok(None)
    }

    pub async fn move_card(
        db: &Arc<PgPool>,
        card_id: i32,
        request: &MoveCardRequest,
    ) -> Result<Option<KanbanCard>> {
        let card = sqlx::query_as!(
            KanbanCard,
            r#"
            UPDATE kanban_card 
            SET column_id = $2,
                position = $3,
                updated_at = $4
            WHERE id = $1
            RETURNING id, column_id, title, description, priority, tags, color, position, 
                      due_date, completed_at, is_archived, created_at, updated_at
            "#,
            card_id,
            request.column_id,
            request.position,
            Utc::now()
        )
        .fetch_optional(db.as_ref())
        .await?;

        Ok(card)
    }

    pub async fn delete_card(db: &Arc<PgPool>, card_id: i32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE kanban_card 
            SET is_archived = true, updated_at = $2
            WHERE id = $1
            "#,
            card_id,
            Utc::now()
        )
        .execute(db.as_ref())
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ==== COMMENT OPERATIONS ====

    pub async fn create_comment(
        db: &Arc<PgPool>,
        card_id: i32,
        user_id: i32,
        request: &CreateCommentRequest,
    ) -> Result<KanbanComment> {
        let comment = sqlx::query_as!(
            KanbanComment,
            r#"
            INSERT INTO kanban_comment (card_id, user_id, content, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, card_id, user_id, content, created_at, updated_at
            "#,
            card_id,
            user_id,
            request.content,
            Utc::now()
        )
        .fetch_one(db.as_ref())
        .await?;

        Ok(comment)
    }

    pub async fn get_comments_by_card(
        db: &Arc<PgPool>,
        card_id: i32,
    ) -> Result<Vec<KanbanComment>> {
        let comments = sqlx::query_as!(
            KanbanComment,
            r#"
            SELECT id, card_id, user_id, content, created_at, updated_at
            FROM kanban_comment 
            WHERE card_id = $1
            ORDER BY created_at ASC
            "#,
            card_id
        )
        .fetch_all(db.as_ref())
        .await?;

        Ok(comments)
    }

    // ==== COMPLEX QUERIES ====

    pub async fn get_board_with_columns_and_cards(
        db: &Arc<PgPool>,
        board_id: i32,
        user_id: i32,
    ) -> Result<Option<BoardWithColumnsAndCards>> {
        // Primeiro, obter o board
        let board = match Self::get_board_by_id(db, board_id, user_id).await? {
            Some(board) => board,
            None => return Ok(None),
        };

        // Obter colunas
        let columns = Self::get_columns_by_board(db, board_id).await?;

        let mut columns_with_cards = Vec::new();

        // Para cada coluna, obter os cards
        for column in columns {
            let cards = Self::get_cards_by_column(db, column.id).await?;
            columns_with_cards.push(ColumnWithCards { column, cards });
        }

        Ok(Some(BoardWithColumnsAndCards {
            board,
            columns: columns_with_cards,
        }))
    }

    pub async fn get_card_with_comments(
        db: &Arc<PgPool>,
        card_id: i32,
    ) -> Result<Option<CardWithComments>> {
        let card = match Self::get_card_by_id(db, card_id).await? {
            Some(card) => card,
            None => return Ok(None),
        };

        let comments = Self::get_comments_by_card(db, card_id).await?;

        // Por enquanto, attachments vazios - implementação futura
        let attachments = Vec::new();

        Ok(Some(CardWithComments {
            card,
            comments,
            attachments,
        }))
    }
}
