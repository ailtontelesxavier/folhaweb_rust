use sqlx::PgPool;

use anyhow::{Ok, Result};
use axum::Json;

use crate::{
    cadastro::{
        model::{Folha, Municipio, Uf},
        repository::{FolhaRepository, MunicipioRepository, UfRepository},
        schema::{CreateFolha, CreateMunicipio, CreateUf, UpdateFolha, UpdateMunicipio, UpdateUf},
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
        Ok(self.repo.get_paginated(pool, find, page, page_size, None).await?)
    }
}

pub struct UfService {
    repo: UfRepository,
}

impl UfService {
    pub fn new() -> Self {
        Self { repo: UfRepository }
    }

    pub async fn get_by_id(&self, pool: &PgPool, id: i32) -> Result<Uf> {
        Ok(self.repo.get_by_id(pool, id).await?)
    }

    pub async fn create(&self, pool: &PgPool, input: CreateUf) -> Result<Uf> {
        Ok(self.repo.create(pool, input).await?)
    }

    pub async fn update(&self, pool: &PgPool, id: i32, input: UpdateUf) -> Result<Uf> {
        Ok(self.repo.update(pool, id, input).await?)
    }

    pub async fn delete(&self, pool: &PgPool, id: i32) -> Result<()> {
        Ok(self.repo.delete(pool, id).await?)
    }

    pub async fn get_paginated(
        &self,
        pool: &PgPool,
        find: Option<&str>,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Uf>> {
        Ok(self.repo.get_paginated(pool, find, page, page_size, None).await?)
    }
}

pub struct MunicipioService {
    repo: MunicipioRepository,
}

impl MunicipioService {
    pub fn new() -> Self {
        Self {
            repo: MunicipioRepository,
        }
    }

    pub async fn get_by_id(&self, pool: &PgPool, id: i32) -> Result<Municipio> {
        Ok(self.repo.get_by_id(pool, id).await?)
    }

    pub async fn create(&self, pool: &PgPool, input: CreateMunicipio) -> Result<Municipio> {
        Ok(self.repo.create(pool, input).await?)
    }

    pub async fn update(
        &self,
        pool: &PgPool,
        id: i32,
        input: UpdateMunicipio,
    ) -> Result<Municipio> {
        Ok(self.repo.update(pool, id, input).await?)
    }

    pub async fn delete(&self, pool: &PgPool, id: i32) -> Result<()> {
        Ok(self.repo.delete(pool, id).await?)
    }

    pub async fn get_paginated(
        &self,
        pool: &PgPool,
        find: Option<&str>,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Municipio>> {
        Ok(self.repo.get_paginated(pool, find, page, page_size, None).await?)
    }
}
