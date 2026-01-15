# Fast Nano CV Engine (Variante In-Memory)

![Versión](https://img.shields.io/badge/version-0.1.1-blue.svg)
![Licencia](https://img.shields.io/badge/license-MIT-green.svg)
![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)
![Rust](https://img.shields.io/badge/rust-1.78.0-orange.svg)

```
    _   __ ___     _   __ ____     ______ _    __
   / | / //   |   / | / // __ \   / ____/| |  / /
  /  |/ // /| |  /  |/ // / / /  / /     | | / /
 / /|  // ___ | / /|  // /_/ /  / /___   | |/ /
/_/ |_//_/  |_|/_/ |_/ \____/   \____/   |___/
```

Un motor de renderizado de CV ultrarrápido escrito en Rust, optimizado como un componente de backend sin estado. Esta variante "in-memory" está diseñada para ser invocada por un orquestador (p. ej., un servicio Nest.js), recibiendo datos por `stdin` y devolviendo un PDF por `stdout`, garantizando máxima velocidad y eficiencia sin acceso a disco.

## 🏛️ Arquitectura y Contexto de Uso

Este proyecto no es una aplicación de usuario final, sino un **componente de backend especializado**. Está diseñado para el siguiente flujo de trabajo:

1.  **Frontend (Next.js):** Un usuario solicita la generación de su CV a través de una interfaz web.
2.  **Orquestador (Nest.js):** El backend recibe la solicitud. Lee los datos del CV del usuario (desde una base de datos o un archivo `.yml`) y los carga en memoria.
3.  **Ejecución del Motor (Este proyecto):** El orquestador invoca el binario compilado de `fast_nano_cv_engine_memory` como un proceso hijo.
    -   Le pasa los datos del CV a través de la **tubería `stdin`**.
    -   Captura el resultado binario (los bytes del archivo PDF) directamente desde la **tubería `stdout`**.
4.  **Respuesta al Cliente:** El orquestador envía el PDF capturado de vuelta al frontend para su descarga o visualización.

Este enfoque evita por completo las operaciones de entrada/salida de disco durante la generación, lo que lo hace ideal para entornos de servidor de alto rendimiento.

## ✨ Características Principales

*   **Procesamiento In-Memory:** Recibe datos YAML por `stdin` y emite un PDF por `stdout`, sin tocar el sistema de archivos.
*   **Sin Estado y Portátil:** El binario es autocontenido, con las plantillas HTML incrustadas, lo que facilita su despliegue.
*   **Rendimiento Nativo:** Construido en Rust para una generación casi instantánea.
*   **Motor de Plantillas:** Personaliza la apariencia de tu CV usando el motor de plantillas [Tera](https://keats.github.io/tera/).
*   **Plantillas Incluidas:** Viene con tres temas listos para usar: `base`, `dark` y `modern`.

## ⚙️ Cómo Funciona

El motor sigue un proceso simple y eficiente optimizado para la integración con otros servicios:

`Datos YAML (vía stdin)` → `Motor Rust` → `Renderizado con Plantilla Incrustada` → `Bytes del PDF (vía stdout)`

## 📋 Prerrequisitos

1.  **Rust y Cargo:** Para compilar el proyecto. Puedes instalarlo desde [rustup.rs](https://rustup.rs/).
2.  **Google Chrome / Chromium:** La generación de PDF depende de `headless_chrome`, por lo que es necesario tener el navegador instalado en el entorno donde se ejecute el binario.

## 🚀 Instalación y Compilación

1.  **Clona el Repositorio:**
    ```bash
    git clone https://github.com/Kakarot-Caracter/fast_nano_cv_engine_memory.git
    cd fast_nano_cv_engine_memory
    ```

2.  **Construye para Producción:**
    Este comando compila el proyecto con optimizaciones.
    ```bash
    cargo build --release
    ```

El binario ejecutable final se ubicará en `target/release/fast_nano_cv_engine_memory`.

## USAGE

El binario está diseñado para ser usado con tuberías (`pipes`). Se le debe pasar el contenido del archivo YAML a través de `stdin` y el PDF resultante será emitido a `stdout`.

### Sintaxis del Comando

```bash
cat <archivo_yaml> | ./target/release/fast_nano_cv_engine_memory [--template <nombre>] > <archivo_salida.pdf>
```

| Argumento              | Descripción                                                                                                |
| ---------------------- | ---------------------------------------------------------------------------------------------------------- |
| `stdin`                | **(Requerido)** Contenido del archivo YAML que se pasa al proceso.                                          |
| `--template <nombre>` | **(Opcional)** El nombre de la plantilla a usar (`base`, `dark`, `modern`). Por defecto, es `base`.     |
| `stdout`               | **(Requerido)** El flujo de salida donde se recibirán los bytes del PDF.                                   |


### Ejemplos de Uso

1.  **Generar CV con la plantilla por defecto (`base`):**
    ```bash
    cat cv.yml | ./target/release/fast_nano_cv_engine_memory > output/giovanni_martinez_cv.pdf
    ```

2.  **Generar CV usando la plantilla `modern`:**
    ```bash
    cat cv.yml | ./target/release/fast_nano_cv_engine_memory --template modern > output/cv_modern.pdf
    ```

3.  **Integración en un script de Node.js (ejemplo para el orquestador):**
    ```javascript
    const { spawn } = require('child_process');
    const fs = require('fs');
    const path = require('path');

    async function generatePdf(yamlData, template = 'base') {
      const binaryPath = path.resolve('./target/release/fast_nano_cv_engine_memory');
      const args = ['--template', template];
      
      return new Promise((resolve, reject) => {
        const process = spawn(binaryPath, args);
        const pdfChunks = [];
        
        process.stdout.on('data', (chunk) => {
          pdfChunks.push(chunk);
        });

        process.stderr.on('data', (data) => {
          // Ideal para logging
          console.error(`[stderr]: ${data}`);
        });

        process.on('close', (code) => {
          if (code === 0) {
            resolve(Buffer.concat(pdfChunks));
          } else {
            reject(new Error(`El proceso terminó con código ${code}`));
          }
        });

        // Escribir los datos YAML en stdin y cerrar la tubería
        process.stdin.write(yamlData);
        process.stdin.end();
      });
    }

    // Uso
    const yamlContent = fs.readFileSync('cv.yml', 'utf-8');
    generatePdf(yamlContent, 'modern').then(pdfBuffer => {
      fs.writeFileSync('cv_from_node.pdf', pdfBuffer);
      console.log('PDF generado desde Node.js!');
    });
    ```

## 📄 Formato del Archivo YAML

El formato del archivo `cv.yml` no ha cambiado. Sigue utilizando la misma estructura para definir las secciones `personal`, `sobre_mi`, `educacion`, `experiencia` y `habilidades`.

*(La sección detallada del formato YAML del README anterior sigue siendo válida y puede consultarse como referencia).*

## 🎨 Plantillas Personalizadas

Para añadir o modificar plantillas:

1.  Edita o añade un nuevo archivo `.html` en la carpeta `src/templates/`.
2.  El sistema `RustEmbed` automáticamente incluirá los cambios en el binario la próxima vez que compiles con `cargo build`.
3.  Ejecuta el programa apuntando a tu nueva plantilla con el flag `--template`.

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Si tienes ideas para mejorar el proyecto, por favor abre un *issue* para discutirlo o envía un *pull request*.

## 📜 Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.
