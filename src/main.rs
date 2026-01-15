use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/templates/"]
struct Asset;

use anyhow::{Context, Result};
use colored::*;
use fast_nano_cv_engine_memory::parser::yaml;
use fast_nano_cv_engine_memory::render::{html, pdf};
use std::env;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let start_time = Instant::now();

    println!(
        "{}",
        r#"
    _   __ ___     _   __ ____     ______ _    __
   / | / //   |   / | / // __ \   / ____/| |  / /
  /  |/ // /| |  /  |/ // / / /  / /     | | / /
 / /|  // ___ | / /|  // /_/ /  / /___   | |/ /
/_/ |_//_/  |_|/_/ |_/ \____/   \____/   |___/
    "#
        .cyan()
        .bold()
    );

    let mut template_name = String::from("base");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--template" => {
                if i + 1 < args.len() {
                    template_name = args[i + 1].clone();
                    i += 2;
                } else {
                    anyhow::bail!("❌ Error: El flag --template requiere un nombre.");
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    let template_content: String = match Asset::get(&format!("{}.html", template_name)) {
        Some(content) => {
            String::from_utf8(content.data.to_vec()).context("El template no es un UTF-8 válido")?
        }
        None => {
            anyhow::bail!("🚫 El template '{}' no existe en el binario", template_name);
        }
    };

    let mut yaml_input = String::new();
    io::stdin()
        .read_to_string(&mut yaml_input)
        .context("No se pudo leer YAML desde stdin")?;

    let cv = yaml::parse_str(&yaml_input)?;

    print!(" {}  {} ", "🛠️ ".yellow(), "Renderizando HTML...".bold());
    let html_content = html::render_html_to_string(&cv, &template_content)?;
    println!("{}", "OK".green());

    print!(" {}  {} ", "⚙️ ".cyan(), "Generando PDF...".bold());
    let pdf_bytes = pdf::html_to_pdf_bytes(&html_content)?;
    println!("{}", "OK".green());

    io::stdout().write_all(&pdf_bytes)?;
    io::stdout().flush()?;

    eprintln!(
        "\n🏆 {} en {:?}",
        "NanoCV completado".green().bold(),
        start_time.elapsed()
    );

    Ok(())
}
