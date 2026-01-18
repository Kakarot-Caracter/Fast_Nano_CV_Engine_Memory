# Fast Nano CV Engine (Versión In-Memory para Node.js)

Un motor de generación de CVs ultrarrápido, escrito en Rust y compilado como un **complemento nativo de Node.js**. Esta versión ha sido modificada del [Fast_Nano_CV_Engine original](https://github.com/Kakarot-Caracter/Fast_Nano_CV_Engine) para operar completamente en memoria, haciéndola ideal para su uso en servidores y backends de alto rendimiento (por ejemplo, con NestJS).

- **Entrada de Datos Flexible**: Procesa un string en formato YAML.
- **Salida en Memoria**: Genera un Buffer de Node.js con el contenido del PDF, sin tocar el sistema de archivos.
- **Rendimiento Nativo**: Construido en Rust y expuesto a través de `N-API` para una comunicación casi instantánea con el runtime de Node.js.
- **Auto-contenido**: Las plantillas HTML están incrustadas en el binario final, simplificando el despliegue.

## Principales Diferencias con la Versión Original

Esta variante fue rediseñada específicamente para casos de uso de backend.

| Característica | Versión Original (CLI) | Esta Versión (Node.js Addon) | Ventaja |
| :--- | :--- | :--- | :--- |
| **Ejecución** | Interfaz de Línea de Comandos (binario) | Importado como un módulo (`.node`) | Integración directa en código JS/TS |
| **Entrada** | Ruta a un archivo `.yml` | `String` con contenido YAML | Evita I/O de disco, ideal para APIs |
| **Salida** | Archivos `.html` y `.pdf` en disco | `Buffer` de Node.js en memoria | Permite enviar el PDF en una respuesta HTTP |
| **Despliegue** | Binario + carpeta de plantillas | Un solo archivo `.node` | Despliegue atómico y simplificado |

---

## 🚀 Uso (Ejemplo con TypeScript/NestJS)

La principal ventaja de este motor es su facilidad de integración. La biblioteca expone una única función asíncrona: `renderCvNative`.

1.  **Importar el Módulo**: Una vez compilado, importa el archivo `.node` en tu servicio.

2.  **Llamar a la función**: Pásale el contenido del CV en un string YAML y el nombre de la plantilla deseada.

```typescript
import { renderCvNative } from '../path/to/your/addon.node';
import { Controller, Post, Body, Res } from '@nestjs/common';
import { Response } from 'express';

// DTO para validar la entrada
class GenerateCvDto {
  template: 'modern' | 'dark' | 'base';
  yamlData: string;
}

@Controller('cv')
export class CvController {
  @Post('generate')
  async generateCv(@Body() body: GenerateCvDto, @Res() res: Response) {
    try {
      // Llama a la función nativa de Rust
      const pdfBuffer: Buffer = await renderCvNative(body.yamlData, body.template);

      // Envía el PDF directamente en la respuesta HTTP
      res.setHeader('Content-Type', 'application/pdf');
      res.setHeader('Content-Disposition', 'attachment; filename=cv.pdf');
      res.send(pdfBuffer);

    } catch (error) {
      // Manejo de errores
      console.error('Error generating CV:', error);
      res.status(500).send({ message: 'Failed to generate CV' });
    }
  }
}
```

---

## ⚙️ Cómo Funciona

El flujo de procesamiento es directo y eficiente:

1.  **Llamada desde Node.js**: Se invoca `renderCvNative(yamlString, templateName)`.
2.  **Parseo**: El string YAML es deserializado por `serde_yaml` en una estructura `CV` de Rust fuertemente tipada.
3.  **Renderizado HTML**: El motor de plantillas `Tera` usa la estructura `CV` para rellenar la plantilla HTML correspondiente (cargada desde los assets embebidos).
4.  **Generación de PDF**: La librería `headless_chrome` se lanza en segundo plano, carga el HTML a través de un data-uri y "imprime" la página a un PDF.
5.  **Retorno**: El contenido del PDF se devuelve como un `Vec<u8>` en Rust, que se convierte en un `Buffer` de Node.js para el consumidor.

---

## 🛠️ Compilación del Proyecto

Para compilar el proyecto y generar el archivo `.node`, necesitas tener instalado `Node.js` y `Rust`.

1.  **Instalar dependencias de N-API**:
    ```bash
    npm install
    ```

2.  **Construir el addon nativo**:
    Este comando compila el código Rust y coloca el archivo `.node` en la raíz del proyecto.
    ```bash
    npx napi build --release
    ```
    Para desarrollo, puedes omitir el flag `--release` para una compilación más rápida pero no optimizada.

El archivo resultante (ej. `fast_nano_cv_engine_memory.node`) es el que debes importar en tu proyecto de Node.js.

## 📁 Estructura del Proyecto

-   `src/lib.rs`: El corazón de la librería. Define la función `render_cv_native` expuesta a Node.js y orquesta todo el flujo.
-   `src/models/cv.rs`: Define las estructuras de datos de Rust (`CV`, `Personal`, etc.) que mapean el formato del YAML.
-   `src/parser/yaml.rs`: Lógica para deserializar el string YAML en las estructuras `CV`.
-   `src/render/html.rs`: Usa `Tera` para renderizar el HTML a partir de los datos.
-   `src/render/pdf.rs`: Convierte el string HTML en un buffer de bytes PDF usando `headless_chrome`.
-   `src/templates/`: Plantillas HTML (`base.html`, `dark.html`, `modern.html`) que se incrustan en el binario final.
-   `Cargo.toml`: Manifiesto del proyecto Rust, donde se definen las dependencias clave como `napi`, `serde`, `tera` y `headless_chrome`.
-   `package.json`: Define las dependencias de desarrollo de Node.js, principalmente para el CLI de `napi-rs`.

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.
