use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct CV {
    pub personal: Personal,
    pub sobre_mi: Option<String>,
    pub experiencia: Vec<Experiencia>,
    pub educacion: Vec<Educacion>,
    pub habilidades: Vec<HashMap<String, String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Personal {
    pub nombre: String,
    pub titulo: String,
    pub telefono: Option<String>,
    pub correo: String,
    pub ubicacion: Option<String>,
    pub web: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Experiencia {
    pub empresa: String,
    pub puesto: String,
    pub inicio: String,
    pub fin: String,
    pub descripcion: Option<String>,
    pub logros: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Educacion {
    pub institucion: String,
    pub grado: String,
    pub inicio: String,
    pub fin: String,
    pub ubicacion: Option<String>,
    pub logros: Vec<String>,
}
