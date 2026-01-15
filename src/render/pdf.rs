use anyhow::{Context, Result};
use headless_chrome::{Browser, LaunchOptions, types::PrintToPdfOptions};
use std::fs;
use std::path::Path;

pub fn html_to_pdf<P: AsRef<Path>>(html_path: P, pdf_path: P) -> Result<()> {
    let html_p = html_path.as_ref();
    let pdf_p = pdf_path.as_ref();

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(true)
            .build()
            .map_err(|e| anyhow::anyhow!(e))?,
    )
    .context("No se pudo iniciar el navegador")?;

    let tab = browser.new_tab().context("No se pudo abrir la pestaña")?;

    let absolute_path = html_p
        .canonicalize()
        .with_context(|| format!("No se pudo encontrar el archivo HTML en: {:?}", html_p))?;

    let url = format!("file://{}", absolute_path.display());

    tab.navigate_to(&url)?.wait_until_navigated()?;

    tab.evaluate(
        "document.body.style.margin = '0'; document.documentElement.style.margin = '0';",
        false,
    )?;

    let scroll_height = tab
        .evaluate("document.body.getBoundingClientRect().height", false)?
        .value
        .context("No se pudo obtener el alto del contenido")?
        .as_f64()
        .unwrap_or(1000.0);

    let height_inches = scroll_height / 96.0;
    let width_inches = 8.27;

    let pdf_options = PrintToPdfOptions {
        print_background: Some(true),
        paper_width: Some(width_inches),
        paper_height: Some(height_inches),
        margin_top: Some(0.0),
        margin_bottom: Some(0.0),
        margin_left: Some(0.0),
        margin_right: Some(0.0),
        prefer_css_page_size: Some(false),
        ..Default::default()
    };

    let pdf_data = tab
        .print_to_pdf(Some(pdf_options))
        .map_err(|e| anyhow::anyhow!("Error al imprimir: {}", e))?;

    fs::write(pdf_p, pdf_data)
        .with_context(|| format!("Error al escribir el archivo PDF en: {:?}", pdf_p))?;

    Ok(())
}
