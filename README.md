# Fast Nano CV Engine

```
    _   __ ___     _   __ ____     ______ _    __
   / | / //   |   / | / // __ \   / ____/| |  / /
  /  |/ // /| |  /  |/ // / / /  / /     | | / /
 / /|  // ___ | / /|  // /_/ /  / /___   | |/ /
/_/ |_//_/  |_|/_/ |_/ \____/   \____/   |___/
```

Un generador de CVs ultrarrápido, simple y elegante a partir de un archivo YAML. Crea un currículum profesional en formato HTML y PDF utilizando plantillas configurables.

## ✨ Características

*   **Generación de HTML y PDF:** Crea una versión web y una en PDF de tu CV.
*   **Basado en YAML:** Define los datos de tu CV en un archivo `cv.yml` simple y legible.
*   **Plantillas personalizables:** Utiliza las plantillas incluidas (`base`, `dark`, `modern`) o crea las tuyas.
*   **Rápido y ligero:** Construido en Rust para un rendimiento óptimo.
*   **Interfaz de línea de comandos (CLI):** Fácil de usar desde la terminal.

## 🚀 Instalación

Asegúrate de tener Rust y Cargo instalados en tu sistema.

1.  **Clona el repositorio:**
    ```bash
    git clone https://github.com/tu-usuario/fast_nano_cv_engine.git
    cd fast_nano_cv_engine
    ```

2.  **Construye el proyecto:**
    ```bash
    cargo build --release
    ```
    El ejecutable se encontrará en `target/release/fast_nano_cv_engine`.

## Usage

Ejecuta el programa desde la raíz del proyecto, proporcionando la ruta a tu archivo de CV en formato YAML.

1.  **Comando básico:**
    Utiliza la plantilla por defecto (`base`):
    ```bash
    cargo run -- cv.yml
    ```

2.  **Usar una plantilla diferente:**
    Especifica el nombre de una plantilla con el flag `--template`. Las plantillas deben estar en el directorio `src/templates`.
    ```bash
    cargo run -- cv.yml --template dark
    ```
    ```bash
    cargo run -- cv.yml --template modern
    ```

Los archivos de salida (`.html` y `.pdf`) se guardarán en el directorio `output/`.

## 🔧 Configuración (cv.yml)

El corazón de este proyecto es tu archivo `cv.yml`. Aquí defines toda la información de tu currículum.

A continuación se muestra un ejemplo de la estructura del archivo `cv.yml`:

```yaml
personal:
  nombre: Tu Nombre Completo
  titulo: Tu Título Profesional
  telefono: "+12 345 67890"
  correo: tu.correo@example.com
  ubicacion: Ciudad, País
  web: "https://tu-sitio-web.com"
  linkedin: "https://linkedin.com/in/tu-usuario"
  github: "https://github.com/tu-usuario"

sobre_mi: >
  Un breve párrafo sobre ti, tus pasiones y lo que te motiva profesionalmente.

educacion:
  - institucion: Nombre de la Institución
    grado: Título Obtenido
    ubicacion: Ciudad, País
    inicio: Mes Año
    fin: Mes Año
    logros:
      - Logro o aprendizaje 1.
      - Logro o aprendizaje 2.

experiencia:
  - empresa: Nombre de la Empresa
    puesto: Tu Cargo
    inicio: Mes Año
    fin: Mes Año
    descripcion: >
      Descripción de tus responsabilidades y del proyecto.
    logros:
      - Logro cuantificable 1.
      - Logro cuantificable 2.

habilidades:
  - Categoría 1: Habilidad A, Habilidad B, Habilidad C
  - Categoría 2: Habilidad D, Habilidad E
```

### Secciones Detalladas:

*   `personal`: Tu información de contacto.
*   `sobre_mi`: Un resumen profesional sobre ti.
*   `educacion`: Tu historial académico. Puedes añadir múltiples entradas.
*   `experiencia`: Tu experiencia laboral. Puedes añadir múltiples entradas.
*   `habilidades`: Una lista de tus habilidades, agrupadas por categorías.

## 🎨 Plantillas

Las plantillas utilizan el motor [Tera](https://keats.github.io/tera/). Puedes crear tus propias plantillas HTML y colocarlas en el directorio `src/templates/`.

Las plantillas incluidas son:
*   `base.html`: Un diseño limpio y estándar.
*   `dark.html`: Un tema oscuro.
*   `modern.html`: Un diseño más contemporáneo.

Para crear tu propia plantilla, simplemente crea un nuevo archivo `.html` en `src/templates/` y úsalo con el flag `--template`.

## 📜 Licencia

Este proyecto no especifica una licencia. Sería una buena idea añadir un archivo `LICENSE` (por ejemplo, MIT, Apache 2.0).
