use anyhow::{Context, Result};
use headless_chrome::{Browser, LaunchOptions, types::PrintToPdfOptions};

/// Convierte HTML en memoria a PDF en memoria (Vec<u8>)
pub fn html_to_pdf_bytes(html_content: &str) -> Result<Vec<u8>> {
    // Iniciar navegador headless
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(true)
            .build()
            .map_err(|e| anyhow::anyhow!(e))?,
    )
    .context("No se pudo iniciar el navegador")?;

    let tab = browser.new_tab().context("No se pudo abrir la pestaña")?;

    // Cargar el HTML directamente usando data URI
    let data_url = format!("data:text/html,{}", urlencoding::encode(html_content));
    tab.navigate_to(&data_url)?.wait_until_navigated()?;

    // Ajuste de márgenes
    tab.evaluate(
        "document.body.style.margin = '0'; document.documentElement.style.margin = '0';",
        false,
    )?;

    // Calcular alto del contenido para el PDF
    let scroll_height = tab
        .evaluate("document.body.getBoundingClientRect().height", false)?
        .value
        .context("No se pudo obtener el alto del contenido")?
        .as_f64()
        .unwrap_or(1000.0);

    let height_inches = scroll_height / 96.0; // 96px por pulgada
    let width_inches = 8.27; // A4

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

    // Generar PDF en memoria
    let pdf_data = tab
        .print_to_pdf(Some(pdf_options))
        .map_err(|e| anyhow::anyhow!("Error al imprimir: {}", e))?;

    Ok(pdf_data)
}
