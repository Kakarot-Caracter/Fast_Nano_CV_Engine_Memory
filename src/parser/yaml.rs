use crate::models::CV;
use anyhow::{Context, Result};
use serde_yaml;

/// Parsear YAML desde un &str (en memoria)
pub fn parse_str(yaml_content: &str) -> Result<CV> {
    let cv: CV =
        serde_yaml::from_str(yaml_content).context("No se pudo parsear el YAML desde memoria")?;
    Ok(cv)
}
