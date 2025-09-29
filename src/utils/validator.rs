use regex::Regex;
use std::sync::LazyLock;
use validator::{ValidationError};

pub static EMAIL_RX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());



pub fn validate_optional_email(email: &Option<String>) -> Result<(), ValidationError> {
    if let Some(e) = email {
        if !EMAIL_RX.is_match(e) {
            return Err(ValidationError::new("invalid_email"));
        }
    }
    Ok(())
}