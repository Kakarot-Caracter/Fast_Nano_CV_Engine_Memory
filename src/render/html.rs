use crate::models::CV;
use anyhow::{Context as AnyhowContext, Result};
use std::path::Path;
use tera::{Context, Tera};

pub fn render_html_from_str<P: AsRef<Path>>(
    cv: &CV,
    template_content: &str,
    output: P,
) -> Result<()> {
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

    std::fs::write(output.as_ref(), rendered).with_context(|| {
        format!(
            "No se pudo escribir el archivo HTML en: {:?}",
            output.as_ref()
        )
    })?;

    Ok(())
}
