use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct KanbanBoard {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_active: bool,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct KanbanColumn {
    pub id: i32,
    pub board_id: i32,
    pub title: String,
    pub position: i32,
    pub color: Option<String>,
    pub max_cards: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct KanbanCard {
    pub id: i32,
    pub column_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub tags: Option<Vec<String>>,
    pub color: Option<String>,
    pub position: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct KanbanComment {
    pub id: i32,
    pub card_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct KanbanAttachment {
    pub id: i32,
    pub card_id: i32,
    pub filename: String,
    pub original_filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub file_path: String,
    pub uploaded_by: i32,
    pub created_at: DateTime<Utc>,
}

// DTOs para criação e atualização
#[derive(Debug, Deserialize)]
pub struct CreateBoardRequest {
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBoardRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub position: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateColumnRequest {
    pub title: String,
    pub color: Option<String>,
    pub max_cards: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateColumnRequest {
    pub title: Option<String>,
    pub position: Option<i32>,
    pub color: Option<String>,
    pub max_cards: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCardRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<Vec<String>>,
    pub color: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<Vec<String>>,
    pub color: Option<String>,
    pub position: Option<i32>,
    pub column_id: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_archived: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MoveCardRequest {
    pub column_id: i32,
    pub position: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

// DTO para resposta completa do board com colunas e cards
#[derive(Debug, Serialize)]
pub struct BoardWithColumnsAndCards {
    pub board: KanbanBoard,
    pub columns: Vec<ColumnWithCards>,
}

#[derive(Debug, Serialize)]
pub struct ColumnWithCards {
    pub column: KanbanColumn,
    pub cards: Vec<KanbanCard>,
}

#[derive(Debug, Serialize)]
pub struct CardWithComments {
    pub card: KanbanCard,
    pub comments: Vec<KanbanComment>,
    pub attachments: Vec<KanbanAttachment>,
}