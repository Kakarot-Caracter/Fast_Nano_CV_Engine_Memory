pub mod errors;
pub mod models;
pub mod parser;
pub mod render;

use napi::{Error, Result as NapiResult, Status};
use napi_derive::napi;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/templates/"]
struct Asset;

#[napi]
pub async fn render_cv_native(yaml_input: String, template_name: String) -> NapiResult<Vec<u8>> {
    let template_file = format!("{}.html", template_name);
    let asset = Asset::get(&template_file).ok_or_else(|| {
        Error::new(
            Status::InvalidArg,
            format!("Template {} no existe", template_name),
        )
    })?;

    let template_content = String::from_utf8(asset.data.to_vec())
        .map_err(|_| Error::new(Status::GenericFailure, "Template UTF-8 inválido"))?;

    let cv = parser::yaml::parse_str(&yaml_input)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Error YAML: {}", e)))?;

    let html_content = render::html::render_html_to_string(&cv, &template_content)
        .map_err(|e| Error::new(Status::GenericFailure, "Error render HTML"))?;

    let pdf_bytes = render::pdf::html_to_pdf_bytes(&html_content)
        .map_err(|e| Error::new(Status::GenericFailure, "Error generar PDF"))?;

    Ok(pdf_bytes)
}
