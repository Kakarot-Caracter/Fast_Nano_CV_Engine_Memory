use crate::errors::ParserError;
use crate::models::CV;
use std::fs;

pub fn parse(path: &str) -> Result<CV, ParserError> {
    let content = fs::read_to_string(path)?;
    let cv = serde_yaml::from_str(&content)?;
    Ok(cv)
}
