use sqlx::PgPool;

use async_trait::async_trait;

use anyhow::Ok;
use anyhow::Result;
use uuid::Uuid;

use crate::cadastro::model::Municipio;
use crate::cadastro::model::Uf;
use crate::cadastro::schema::CreateMunicipio;
use crate::cadastro::schema::CreateUf;
use crate::cadastro::schema::UpdateMunicipio;
use crate::cadastro::schema::UpdateUf;
use crate::{
    cadastro::{
        model::Folha,
        schema::{CreateFolha, UpdateFolha},
    },
    repository::Repository,
};

pub struct FolhaRepository;

#[async_trait]
impl Repository<Folha, i64> for FolhaRepository {
    type CreateInput = CreateFolha;
    type UpdateInput = UpdateFolha;

    fn table_name(&self) -> &str {
        "cadastro_folha f"
    }

    fn id_column(&self) -> &str {
        "f.id"
    }

    fn order_by_column(&self) -> &str {
        "f.ano DESC, f.mes ASC"
    }

    fn searchable_fields(&self) -> &[(&str, &str)] {
        &[
            ("f.ano", "="),
            ("f.mes", "="),
            ("serv.nome", "ILIKE"),
        ]
    }

    fn select_clause(&self) -> &str {
        "f.id, f.orgao_id, f.ano, f.mes, 
        f.servidor_id, f.salario, f.base_fgts, f.base_inss,
        f.base_irrf, f.ded_irrf, f.cargo_id, f.setor_id,
        f.departamento_id, f.vinculo_id, serv.nome as serv_nome, org.nome as org_nome"
    }

    fn from_clause(&self) -> &str {
        "cadastro_folha f
        INNER JOIN cadastro_servidor serv ON serv.id = f.servidor_id
        INNER JOIN cadastro_orgao org ON org.id = f.orgao_id
        "
    }

    async fn create(&self, pool: &PgPool, input: Self::CreateInput) -> Result<Folha> {
        Ok(sqlx::query_as!(
            Folha,
            r#"INSERT INTO cadastro_folha(
            orgao_id, ano, mes, servidor_id, salario, base_fgts, base_inss, base_irrf, ded_irrf, cargo_id, setor_id, departamento_id, vinculo_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) RETURNING *, NULL as "serv_nome?", NULL as "org_nome?" "#,
            input.orgao_id,
            input.ano,
            input.mes,
            input.servidor_id,
            input.salario,
            input.base_fgts,
            input.base_inss,
            input.base_irrf,
            input.ded_irrf,
            input.cargo_id,
            input.setor_id,
            input.departamento_id,
            input.vinculo_id
        )
        .fetch_one(pool)
        .await?)
    }

    async fn update(&self, pool: &PgPool, id: i64, input: Self::UpdateInput) -> Result<Folha> {
        Ok(sqlx::query_as!(
            Folha,
            r#"
            UPDATE cadastro_folha
            SET
                orgao_id = COALESCE($1, orgao_id),
                ano = COALESCE($2, ano),
                mes = COALESCE($3, mes),
                servidor_id = COALESCE($4, servidor_id),
                salario = COALESCE($5, salario),
                base_fgts = COALESCE($6, base_fgts),
                base_inss = COALESCE($7, base_inss),
                base_irrf = COALESCE($8, base_irrf),
                ded_irrf = COALESCE($9, ded_irrf),
                cargo_id = COALESCE($10, cargo_id),
                setor_id = COALESCE($11, setor_id),
                departamento_id = COALESCE($12, departamento_id),
                vinculo_id = COALESCE($13, vinculo_id)
            WHERE id = $14
            RETURNING *, NULL as "serv_nome?", NULL as "org_nome?" "#,
            input.orgao_id,
            input.ano,
            input.mes,
            input.servidor_id,
            input.salario,
            input.base_fgts,
            input.base_inss,
            input.base_irrf,
            input.ded_irrf,
            input.cargo_id,
            input.setor_id,
            input.departamento_id,
            input.vinculo_id,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    async fn delete(&self, pool: &PgPool, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM cadastro_folha WHERE id = $1", id as i32)
            .execute(pool)
            .await?;
        Ok(())
    }
}


pub struct UfRepository;

#[async_trait]
impl Repository<Uf, i32> for UfRepository {
    type CreateInput = CreateUf;
    type UpdateInput = UpdateUf;

    fn table_name(&self) -> &str {
        "cadastro_uf u"
    }

    fn id_column(&self) -> &str {
        "u.id"
    }

    fn order_by_column(&self) -> &str {
        "u.sigla DESC"
    }

    fn searchable_fields(&self) -> &[(&str, &str)] {
        &[
            ("u.sigla", "ILIKE"),
            ("u.nome", "ILIKE"),
        ]
    }

    fn select_clause(&self) -> &str {
        "u.id, u.sigla, u.nome"
    }

    fn from_clause(&self) -> &str {
        "cadastro_uf u"
    }

    async fn create(&self, pool: &PgPool, input: Self::CreateInput) -> Result<Uf> {
        Ok(sqlx::query_as!(
            Uf,
            r#"INSERT INTO cadastro_uf(sigla, nome)
            VALUES ($1, $2) RETURNING *"#,
            input.sigla,
            input.nome
        )
        .fetch_one(pool)
        .await?)
    }

    async fn update(&self, pool: &PgPool, id: i32, input: Self::UpdateInput) -> Result<Uf> {
        Ok(sqlx::query_as!(
            Uf,
            r#"
            UPDATE cadastro_uf
            SET
                sigla = COALESCE($1, sigla),
                nome = COALESCE($2, nome)
            WHERE id = $3
            RETURNING *"#,
            input.sigla,
            input.nome,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    async fn delete(&self, pool: &PgPool, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM cadastro_uf WHERE id = $1", id as i32)
            .execute(pool)
            .await?;
        Ok(())
    }
}


pub struct MunicipioRepository;

#[async_trait]
impl Repository<Municipio, i32> for MunicipioRepository {
    type CreateInput = CreateMunicipio;
    type UpdateInput = UpdateMunicipio;

    fn table_name(&self) -> &str {
        "cadastro_municipio m"
    }

    fn id_column(&self) -> &str {
        "m.id"
    }

    fn order_by_column(&self) -> &str {
        "u.nome ASC, m.nome ASC"
    }

    fn searchable_fields(&self) -> &[(&str, &str)] {
        &[
            ("m.nome", "ILIKE"),
            ("u.nome", "ILIKE"),
        ]
    }

    fn select_clause(&self) -> &str {
        "m.id, m.uf_id, m.nome, u.nome AS uf_nome"
    }

    fn from_clause(&self) -> &str {
        "cadastro_municipio m
        INNER JOIN cadastro_uf u ON u.id = m.uf_id
        "
    }

    async fn create(&self, pool: &PgPool, input: Self::CreateInput) -> Result<Municipio> {
        Ok(sqlx::query_as!(
            Municipio,
            r#"INSERT INTO cadastro_municipio(uf_id, nome)
            VALUES ($1, $2) RETURNING *, NULL as "uf_nome?" "#,
            input.uf_id,
            input.nome
        )
        .fetch_one(pool)
        .await?)
    }

    async fn update(&self, pool: &PgPool, id: i32, input: Self::UpdateInput) -> Result<Municipio> {
        Ok(sqlx::query_as!(
            Municipio,
            r#"
            UPDATE cadastro_municipio
            SET
                nome = COALESCE($1, nome)
            WHERE id = $2
            RETURNING *, NULL as "uf_nome?" "#,
            input.nome,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    async fn delete(&self, pool: &PgPool, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM cadastro_municipio WHERE id = $1", id as i32)
            .execute(pool)
            .await?;
        Ok(())
    }
}

