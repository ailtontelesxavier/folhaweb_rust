use sqlx::PgPool;

use anyhow::{Ok, Result};
use axum::Json;

use crate::{
    folha::{
        model::Folha,
        repository::FolhaRepository,
        schema::{CreateFolha, UpdateFolha},
    },
    repository::{PaginatedResponse, Repository},
};

pub struct FolhaService {
    repo: FolhaRepository,
}

impl FolhaService {
    pub fn new() -> Self {
        Self {
            repo: FolhaRepository,
        }
    }

    pub async fn get_by_id(&self, pool: &PgPool, id: i64) -> Result<Folha> {
        Ok(self.repo.get_by_id(pool, id).await?)
    }

    pub async fn create(&self, pool: &PgPool, input: CreateFolha) -> Result<Folha> {
        Ok(self.repo.create(pool, input).await?)
    }

    pub async fn update(&self, pool: &PgPool, id: i64, input: UpdateFolha) -> Result<Folha> {
        Ok(self.repo.update(pool, id, input).await?)
    }

    pub async fn delete(&self, pool: &PgPool, id: i64) -> Result<()> {
        Ok(self.repo.delete(pool, id).await?)
    }

    pub async fn get_paginated(
        &self,
        pool: &PgPool,
        find: Option<&str>,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Folha>> {
        Ok(self.repo.get_paginated(pool, find, page, page_size).await?)
    }
}
