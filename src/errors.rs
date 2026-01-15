use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("No se pudo leer el archivo")]
    Io(#[from] std::io::Error),

    #[error("YAML inválido")]
    Yaml(#[from] serde_yaml::Error),
}
