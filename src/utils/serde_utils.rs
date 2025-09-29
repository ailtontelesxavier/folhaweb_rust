use std::str::FromStr;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Deserializer, Serializer};
/*
utilizado nos shemas para converter checkbox em booleano
*/
pub fn bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(matches!(
        opt.as_deref(),
        Some("true") | Some("on") | Some("1") | Some("yes")
    ))
}

// versão para Option<bool>
pub fn option_bool_from_str<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.map(|s| matches!(s.as_str(), "true" | "on" | "1" | "yes")))
}

/*
utilizado no formulario html com formato que trata.
20.000,00 → 20000.00.
*/
pub fn brl_to_bigdecimal<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    // remove pontos de milhar e troca vírgula por ponto
    let normalized = s.replace(".", "").replace(",", ".");
    BigDecimal::from_str(&normalized).map_err(serde::de::Error::custom)
}


pub fn de_string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i32>().map_err(serde::de::Error::custom)
}

pub fn de_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i64>().map_err(serde::de::Error::custom)
}


pub fn de_opt_string_to_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if !v.trim().is_empty() => v
            .parse::<i32>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        _ => Ok(None),
    }
}

pub fn option_string_as_empty<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(s) => serializer.serialize_str(s),
        None => serializer.serialize_str(""),
    }
}

pub fn empty_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}