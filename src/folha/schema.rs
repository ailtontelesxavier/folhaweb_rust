use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolha {
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
}

/// Schema para atualizar parcialmente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFolha {
    pub id: i64,
    pub orgao_id: i32,
    pub ano: i32,
    pub mes: i32,
    pub servidor_id: i32,
    pub salario: Option<BigDecimal>,
    pub base_fgts: Option<BigDecimal>,
    pub base_inss: Option<BigDecimal>,
    pub base_irrf: Option<BigDecimal>,
    pub ded_irrf: Option<BigDecimal>,
    pub cargo_id: Option<i32>,
    pub setor_id: Option<i32>,
    pub departamento_id: Option<i32>,
    pub vinculo_id: Option<i32>,
}
