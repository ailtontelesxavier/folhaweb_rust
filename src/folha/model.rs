use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct  Folha {
    pub id: i64,
    pub orgao_id: i32,
    pub ano: i32,
    pub mes: i32,
    pub servidor_id: i32,
    pub salario: BigDecimal,
    pub base_fgts: BigDecimal,
    pub base_inss: BigDecimal,
    pub base_irrf: BigDecimal,
    pub ded_irrf: BigDecimal,
    pub cargo_id: i32,
    pub setor_id: i32,
    pub departamento_id: i32,
    pub vinculo_id: i32,

    // campos de outra tabela
    pub serv_nome: Option<String>,
    pub org_nome: Option<String>,
}
