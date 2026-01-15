use crate::models::CV;
use anyhow::Result;
use tera::{Context, Tera};

/// Renderiza HTML desde un CV y un template, devuelve el HTML como String en memoria
pub fn render_html_to_string(cv: &CV, template_content: &str) -> Result<String> {
    let mut tera = Tera::default();

    tera.add_raw_template("cv_template", template_content)
        .map_err(|e| anyhow::anyhow!("Error al cargar el template interno: {}", e))?;

    let mut context = Context::new();
    context.insert("personal", &cv.personal);
    context.insert("sobre_mi", &cv.sobre_mi);
    context.insert("experiencia", &cv.experiencia);
    context.insert("educacion", &cv.educacion);
    context.insert("habilidades", &cv.habilidades);
    context.insert("extra", &cv.extra);

    let rendered = tera
        .render("cv_template", &context)
        .map_err(|e| anyhow::anyhow!("Error al renderizar con Tera: {}", e))?;

    Ok(rendered)
}
