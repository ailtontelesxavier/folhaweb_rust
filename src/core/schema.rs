use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use regex::Regex;
use sqlx::FromRow;
use std::sync::LazyLock;
use validator::Validate;

use crate::utils::serde_utils::bool_from_str;

static EMAIL_RX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

/*
mínimo 6 caracteres
pelo menos 1 letra maiúscula
pelo menos 1 caractere especial (não alfanumérico, tipo !@#$%&* etc)
-------------------------
^ e $ → início e fim da string (garante que a senha toda seja validada).
(?=.*[A-Z]) → lookahead que exige pelo menos uma letra maiúscula.
(?=.*[^a-zA-Z0-9]) → lookahead que exige pelo menos um caractere especial (qualquer coisa fora de letras/números).
.{6,} → comprimento mínimo de 6 caracteres.
*/
static PASSWORD_RX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?=.*[A-Z])(?=.*[^a-zA-Z0-9]).{6,}$").unwrap());

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateModuleSchema {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateModuleSchema {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PermissionModuloSchema {
    pub id: i32,
    pub name: String,
    pub module_id: i32,
    pub module_title: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionCreateSchema {
    pub name: String,
    pub description: Option<String>,
    pub module_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionUpdateSchema {
    pub name: String,
    pub description: Option<String>,
    pub module_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerfilCreateSchema {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerfilUpdateSchema {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 8, message = "Password must have at least 8 characters"))]
    pub password: String,
    pub last_login: DateTime<Utc>,
    #[serde(default, deserialize_with = "bool_from_str")]
    pub is_superuser: bool,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[validate(regex(path=*EMAIL_RX, message = "Invalid email"))]
    pub email: String,
    #[serde(default, deserialize_with = "bool_from_str")]
    pub is_staff: bool,
    #[serde(default, deserialize_with = "bool_from_str")]
    pub is_active: bool
}

    

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUser {
    pub id: i64,
    #[validate(length(min = 8, message = "Password must have at least 8 characters"))]
    pub password: String,
    pub last_login: DateTime<Utc>,
    #[serde(default, deserialize_with = "bool_from_str")]
    pub is_superuser: bool,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[validate(regex(path=*EMAIL_RX, message = "Invalid email"))]
    pub email: String,
    #[serde(default, deserialize_with = "bool_from_str")]
    pub is_staff: bool,
    #[serde(default, deserialize_with = "bool_from_str")]
    pub is_active: bool
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDtoSchema {
    #[validate(length(min = 6, message = "new password must be at least 6 characters"))]
    pub new_password: String,

    #[validate(
        length(
            min = 6,
            message = "new password confirm must be at least 6 characters"
        ),
        must_match(other = "new_password", message = "new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(length(min = 6, message = "Old password must be at least 6 characters"))]
    pub old_password: String,
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UpdateUserPassword {
    #[validate(length(min = 6, message = "new password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UpdateUserLocalPassword {
    pub password: String,
    #[validate(regex(path = *PASSWORD_RX, message = "A senha deve ter no mínimo 6 caracteres, incluir 1 letra maiúscula e 1 caractere especial."))]
    pub new_password: String,
}

/*
Utilizado para passar o id via parametro GET
*/
#[derive(Deserialize)]
pub struct UserParams {
    pub user_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRolesCreateSchema {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRolesUpdateSchema {
    pub id: i32,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRolesViewSchema {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub name: String, //name no perfil(role)
}

#[derive(Deserialize)]
pub struct IdParams {
    pub id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RolePermissionCreateSchema {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RolePermissionUpdateSchema {
    pub id: i64,
    pub role_id: Option<i32>,
    pub permission_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RolePermissionViewSchema {
    pub id: i64,
    pub role_id: i32,
    pub permission_id: i32,
    pub name: String, //name  permissao
}

/*

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserCreateSchema {
    pub username: String,
    #[validate(length(min = 8, message = "Password must have at least 8 characters"))]
    pub password: String,
    #[validate(regex(path=*EMAIL_RX, message = "Invalid email"))]
    pub email: String,
    pub full_name: String,
    pub otp_base32: Option<String>,
    #[serde(deserialize_with = "checkbox_bool")]
    pub is_active: bool,
    #[serde(deserialize_with = "checkbox_bool")]
    pub is_staff: bool,
    #[serde(deserialize_with = "checkbox_bool")]
    pub is_superuser: bool,
}

*/
