# Sistema Planetario con Shaders en Rust

Este proyecto es un **renderer por software** escrito en Rust que muestra un pequeño sistema solar 2D usando **ray tracing de esferas** y **shaders procedurales** (sin texturas).

Todo el color de los planetas y anillos se genera mediante funciones matemáticas sobre la normal de la esfera y el tiempo.

---

## Cuerpos celestes implementados

El sistema incluye:

- ☀️ **Sol**  
  - Shader emisivo (no depende de una luz externa).  
  - Centro muy brillante casi blanco y bordes más anaranjados.  
  - Granulación suave animada para simular la superficie solar.

- 🌍 **Tierra**  
  - Océanos con gradiente de azules.  
  - Continentes en tonos verdes y marrones.  
  - Polos helados y una capa suave tipo nubes.  
  - Rotación de la “textura” para simular el giro del planeta.

- 🔴 **Marte**  
  - Superficie rojiza con zonas oscuras.  
  - Casquetes polares claros.  
  - Rotación lenta de la superficie.

- ☿️ **Mercurio**  
  - Superficie gris rocosa.  
  - Patrón de cráteres.  
  - Rotación rápida.

- ♄ **Saturno**  
  - Gigante gaseoso con bandas de color beige/marrón.  
  - Swirls/sutilezas en longitud para variar la superficie.  
  - Rotación propia del shader.

- 💍 **Anillos de Saturno**  
  - Modelo separado (plano con agujero, no parte de la esfera).  
  - Colores beige/gris con bandas según el radio.  
  - Iluminados desde el Sol, siguiendo la posición de Saturno.

---

## Movimiento (rotación y traslación)

El sistema incluye animaciones:

- **Traslación (órbitas)**  
  - Mercurio, Tierra, Marte y Saturno orbitan alrededor del Sol en el plano XZ.  
  - Cada planeta tiene un radio de órbita y una velocidad angular distinta.

- **Rotación**  
  - Los shaders de Tierra, Marte, Mercurio y Saturno usan el tiempo para rotar sus patrones, simulando la rotación sobre su eje.

---

## Tecnologías utilizadas

- [Rust](https://www.rust-lang.org/)
- [raylib-rs](https://github.com/deltaphc/raylib-rs) como backend de ventana y dibujo de píxeles.
- Shaders procedurales implementados a mano, sin texturas ni materiales cargados desde archivos.

Archivos principales:

- `src/main.rs` — Bucle principal, ray tracing, órbitas, intersección con esferas y anillos.
- `src/geom.rs` — Tipos y helpers de vectores (`Vec3`, `vec3`, etc.).
- `src/body.rs` — Definición de `CelestialBody` y `BodyShader`.
- `src/shaders.rs` — Implementación de los shaders del Sol, planetas y anillos.

---

## Cómo ejecutar

1. Tener instalado Rust y Cargo:  
   <https://www.rust-lang.org/tools/install>

2. Clonar el repositorio y entrar a la carpeta del proyecto:

   ```bash
   git clone <url-del-repo>
   cd <carpeta-del-proyecto>


3. cargo run --release   
