use sqlx::PgPool;

use async_trait::async_trait;

use anyhow::Ok;
use anyhow::Result;
use uuid::Uuid;

use crate::{
    core::{
        model::User,
        schema::{CreateUser, UpdateUser},
    },
    repository::Repository,
};

pub struct UserRepository;

#[async_trait]
impl Repository<User, i64> for UserRepository {
    type CreateInput = CreateUser;
    type UpdateInput = UpdateUser;

    fn table_name(&self) -> &str {
        "auth_user u"
    }

    fn searchable_fields(&self) -> &[(&str, &str)] {
        &[
            ("u.username", "ILIKE"),
            ("u.email", "ILIKE"),
            ("u.first_name", "ILIKE"),
            ("u.last_name", "ILIKE"),
        ]
    }

    fn select_clause(&self) -> &str {
        "u.id, u.password, u.last_login, u.is_superuser, u.username, u.first_name, u.last_name, u.email, u.is_staff, u.is_active, u.date_joined"
    }

    fn from_clause(&self) -> &str {
        "auth_user u"
    }

    async fn create(&self, pool: &PgPool, input: Self::CreateInput) -> anyhow::Result<User> {
        // 1. Checar se já existe usuário com email ou username
        if let Some(db_user) = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM auth_user
            WHERE email = $1 OR username = $2
            "#,
            input.email,
            input.username
        )
        .fetch_optional(pool)
        .await?
        {
            if db_user.email == input.email {
                anyhow::bail!("Email already registered");
            } else {
                anyhow::bail!("Username already registered");
            }
        }

        // 2. Inserir novo usuário
        let new_user = sqlx::query_as!(
            User,
            r#"INSERT INTO auth_user (
                password,
                is_superuser,
                username,
                first_name,
                last_name,
                email,
                is_staff,
                is_active,
                date_joined
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW()) 
            RETURNING * "#,
            input.password,
            input.is_superuser,
            input.username,
            input.first_name,
            input.last_name,
            input.email,
            input.is_staff,
            input.is_active,
        )
        .fetch_one(pool)
        .await?;

        Ok(new_user)
    }

    async fn update(&self, pool: &PgPool, id: i64, input: Self::UpdateInput) -> Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
            UPDATE auth_user
            SET 
                password = COALESCE($1, password),
                is_superuser  = COALESCE($2, is_superuser),
                username = COALESCE($3, username),
                first_name = COALESCE($4, first_name),
                last_name = COALESCE($5, last_name),
                email = COALESCE($6, email),
                is_staff = COALESCE($7, is_staff),
                is_active = COALESCE($8, is_active),
                last_login = COALESCE($9, last_login)
            WHERE id = $10
            RETURNING * "#,
            input.password,
            input.is_superuser,
            input.username,
            input.first_name,
            input.last_name,
            input.email,
            input.is_staff,
            input.is_active,
            input.last_login,
            id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    async fn delete(&self, pool: &PgPool, id: i64) -> Result<()> {
        sqlx::query!(r#"DELETE FROM auth_user WHERE id = $1"#, id as i64)
            .execute(pool)
            .await?;
        Ok(())
    }
}
