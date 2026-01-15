use anyhow::{Context, Result};
use colored::*;
use fast_nano_cv_engine::parser::yaml;
use fast_nano_cv_engine::render::{html, pdf};
use std::env;
use std::fs;
use std::path::PathBuf;
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

    if args.len() < 2 {
        println!(
            "{} Uso: fast_nano_cv \"<archivo.yml>\" [--template <nombre>]",
            "💡".yellow()
        );
        return Ok(());
    }

    let mut input_path = PathBuf::new();
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
                input_path = PathBuf::from(&args[i]);
                i += 1;
            }
        }
    }

    let template_owned_content: String; // Para almacenar contenido leído de disco

    let template_content: &str = match template_name.as_str() {
        "base" => include_str!("templates/base.html"),
        // Si no es "base", buscamos un archivo .html con ese nombre
        _ => {
            let custom_path =
                PathBuf::from("src/templates").join(format!("{}.html", template_name));
            if custom_path.exists() {
                template_owned_content = fs::read_to_string(&custom_path).with_context(|| {
                    format!("No se pudo leer el archivo de template: {:?}", custom_path)
                })?;
                &template_owned_content
            } else {
                anyhow::bail!(
                    "🚫 El template '{}' no existe en src/templates/ ni está integrado.",
                    template_name
                );
            }
        }
    };

    let output_dir = PathBuf::from("output");
    if !output_dir.exists() {
        fs::create_dir(&output_dir).context("No se pudo crear la carpeta 'output'")?;
    }

    let file_stem = input_path
        .file_stem()
        .context("Archivo de entrada inválido")?;
    let html_path = output_dir.join(format!("{}.html", file_stem.to_string_lossy()));
    let pdf_path = output_dir.join(format!("{}_CV.pdf", file_stem.to_string_lossy()));

    println!(
        " {}  {} {}",
        "📖".blue(),
        "Datos:".bold(),
        input_path.display()
    );
    println!(
        " {}  {} {}",
        "📦".magenta(),
        "Template:".bold(),
        format!("Interno ({})", template_name).bright_black()
    );

    let cv = yaml::parse(input_path.to_str().context("Error en la ruta del YAML")?)?;

    print!(" {}  {} ", "🛠️ ".yellow(), "Renderizando HTML...".bold());

    html::render_html_from_str(&cv, template_content, &html_path)?;
    println!("{}", "OK".green());

    print!(" {}  {} ", "⚙️ ".cyan(), "Generando PDF...".bold());

    pdf::html_to_pdf(&html_path, &pdf_path)?;
    println!("{}", "OK".green());

    println!(
        "\n🏆 {} en {:?}",
        "NanoCV completado".green().bold(),
        start_time.elapsed()
    );
    println!(
        "📂 Resultado: {}\n",
        pdf_path.display().to_string().bright_white().underline()
    );

    Ok(())
}
