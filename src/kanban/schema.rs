use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBoardSchema {
    #[validate(length(
        min = 1,
        max = 255,
        message = "O título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: String,

    #[validate(length(max = 1000, message = "A descrição deve ter no máximo 1000 caracteres"))]
    pub description: Option<String>,

    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBoardSchema {
    #[validate(length(
        min = 1,
        max = 255,
        message = "O título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: Option<String>,

    #[validate(length(max = 1000, message = "A descrição deve ter no máximo 1000 caracteres"))]
    pub description: Option<String>,

    pub color: Option<String>,
    pub position: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateColumnSchema {
    #[validate(length(
        min = 1,
        max = 255,
        message = "O título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: String,

    pub color: Option<String>,

    #[validate(range(min = 1, message = "O limite máximo de cards deve ser pelo menos 1"))]
    pub max_cards: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateColumnSchema {
    #[validate(length(
        min = 1,
        max = 255,
        message = "O título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: Option<String>,

    pub position: Option<i32>,
    pub color: Option<String>,

    #[validate(range(min = 1, message = "O limite máximo de cards deve ser pelo menos 1"))]
    pub max_cards: Option<i32>,

    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCardSchema {
    #[validate(length(
        min = 1,
        max = 255,
        message = "O título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: String,

    #[validate(length(max = 2000, message = "A descrição deve ter no máximo 2000 caracteres"))]
    pub description: Option<String>,

    #[validate(custom(
        function = "validate_priority",
        message = "Prioridade deve ser: low, medium, high ou urgent"
    ))]
    pub priority: Option<String>,

    pub tags: Option<Vec<String>>,
    pub color: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCardSchema {
    #[validate(length(
        min = 1,
        max = 255,
        message = "O título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: Option<String>,

    #[validate(length(max = 2000, message = "A descrição deve ter no máximo 2000 caracteres"))]
    pub description: Option<String>,

    #[validate(custom(
        function = "validate_priority",
        message = "Prioridade deve ser: low, medium, high ou urgent"
    ))]
    pub priority: Option<String>,

    pub tags: Option<Vec<String>>,
    pub color: Option<String>,
    pub position: Option<i32>,
    pub column_id: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_archived: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MoveCardSchema {
    pub column_id: i32,
    pub position: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentSchema {
    #[validate(length(
        min = 1,
        max = 2000,
        message = "O comentário deve ter entre 1 e 2000 caracteres"
    ))]
    pub content: String,
}

// Resposta padrão para APIs
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: "Operação realizada com sucesso".to_string(),
            data: Some(data),
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message,
            data: None,
        }
    }
}

// Função auxiliar para validação de cor hexadecimal
pub fn is_valid_hex_color(color: &str) -> bool {
    if !color.starts_with('#') || color.len() != 7 {
        return false;
    }
    color[1..].chars().all(|c| c.is_ascii_hexdigit())
}

// Função de validação customizada para prioridade
fn validate_priority(priority: &str) -> Result<(), validator::ValidationError> {
    match priority {
        "low" | "medium" | "high" | "urgent" => Ok(()),
        _ => {
            let mut error = validator::ValidationError::new("invalid_priority");
            error.message = Some("Prioridade deve ser: low, medium, high ou urgent".into());
            Err(error)
        }
    }
}

// Estruturas para filtros e paginação
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_limit")]
    pub limit: u32,
}

#[derive(Debug, Deserialize)]
pub struct BoardFilters {
    pub search: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CardFilters {
    pub priority: Option<String>,
    pub tag: Option<String>,
    pub overdue: Option<bool>,
}

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    20
}
