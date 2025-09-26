use sqlx::PgPool;

use async_trait::async_trait;

use anyhow::Ok;
use anyhow::Result;
use uuid::Uuid;

use crate::{
    folha::{
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

    fn searchable_fields(&self) -> &[&str] {
        &["f.ano", "f.mes"]
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
