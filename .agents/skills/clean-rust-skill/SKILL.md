---
name: clean-rust-skill
description: Skill especializada en aplicar principios de Clean Code y buenas prácticas de Rust en proyectos full-stack. Combina clean-code y rust-best-practices.
---

# CleanRustSkill

CleanRustSkill es una skill especializada en aplicar principios de código limpio y buenas prácticas de Rust dentro de proyectos full-stack. Su objetivo es ayudar a que el código generado para NebriPop sea claro, modular, mantenible, seguro, idiomático y alineado con una arquitectura limpia.

Esta skill debe servir tanto para tareas de implementación como para tareas de revisión técnica, refactorización, validación de calidad y documentación de decisiones de código.

## Ámbito de aplicación
- Código backend en Rust.
- Código frontend en Rust.
- Servicios, Repositorios, Handlers, Modelos, DTOs, Componentes frontend.
- Validaciones, Gestión de errores, Tests, Refactorizaciones.
- Revisión de arquitectura, mantenibilidad, legibilidad, duplicidades y buenas prácticas de Rust.

## Agentes que podrán utilizar esta skill
- RustArchitectAgent.
- RustBackend.
- RustFrontend.
- ReviewerAgent.
- QAAgent (al revisar calidad de tests).
- DevopsAgent (al revisar comandos de validación de Cargo).

## Cualidades heredadas de clean-code
- Código claro y fácil de leer.
- Nombres descriptivos para variables, funciones, módulos y estructuras.
- Funciones pequeñas y con una única responsabilidad.
- Evitar duplicación de código y código muerto.
- Evitar comentarios innecesarios cuando el código puede explicarse por sí mismo.
- Separación clara de responsabilidades, bajo acoplamiento y alta cohesión.
- Organización lógica de archivos y módulos.
- Código fácil de testear y fácil de mantener.
- Evitar sobreingeniería, priorizar simplicidad.
- Evitar funciones demasiado largas o estructuras complejas.
- Mantener consistencia de estilo.
- Refactorizar cuando el código se vuelva difícil de entender.
- Documentar decisiones importantes.

## Cualidades heredadas de rust-best-practices
- Uso correcto del sistema de tipos de Rust.
- Uso adecuado de `Result` y `Option`. Gestión explícita de errores.
- Evitar `unwrap()` y `expect()` innecesarios en lógica crítica.
- Uso adecuado de ownership y borrowing. Evitar clones innecesarios.
- Uso correcto de lifetimes.
- Preferir código idiomático de Rust.
- Uso de structs y enums de forma clara, y traits cuando aporten valor real.
- Separación entre lógica de dominio, infraestructura y presentación.
- Uso correcto de async/await si aplica.
- Manejo seguro de concurrencia.
- Validación clara de entradas. Evitar panics en producción.
- Usar `cargo fmt`, `cargo clippy`, `cargo check` y `cargo test`.
- Crear tests unitarios y de integración.

## Principios Combinados
- Código Rust limpio, idiomático, mantenible, seguro, modular y fácil de probar.
- Código alineado con Clean Architecture y sin complejidad accidental ni duplicidad.
- Código adecuado para un MVP académico pero con calidad profesional.

## Reglas de comportamiento
Cuando un agente utilice CleanRustSkill, debe:
1. Revisar siempre el contexto del proyecto antes de proponer cambios.
2. Mantener el código alineado con el PRD y el IMPLEMENTATION_PLAN.md.
3. Respetar RULES.md, priorizar el MVP y evitar sobreingeniería.
4. No introducir dependencias innecesarias ni cambiar la arquitectura sin autorización.
5. No modificar archivos fuera del alcance de la tarea.
6. Explicar los cambios antes de realizarlos y resumirlos después de realizarlos.
7. Recomendar pruebas o comandos de validación.

## Antipatrones que debe evitar
- Código duplicado, funciones o módulos demasiado largos.
- Nombres confusos, acoplamiento excesivo.
- Lógica de negocio mezclada con infraestructura o frontend.
- Uso excesivo de `unwrap()`, clones innecesarios, abstracciones prematuras.
- Errores silenciosos, panics innecesarios y funcionalidades fuera del MVP.

## Aplicación en Backend y Frontend Rust
- **Backend:** Ayuda a crear/revisar rutas limpias, handlers simples, servicios y repositorios separados, modelos/DTOs bien definidos y una gestión de errores explícita integrada con SQLite.
- **Frontend:** Ayuda a crear/revisar componentes reutilizables, vistas claras, llamadas API y gestión de estado limpias, y un buen manejo de cargas y errores.

## Formatos de Respuesta Recomendados
**Antes de modificar código:**
1. Objetivo del cambio.
2. Archivos que va a revisar / modificar.
3. Principios Clean Code y buenas prácticas Rust que aplicará.
4. Riesgos detectados y pruebas a ejecutar.

**Después de modificar código:**
1. Archivos modificados y cambios realizados.
2. Mejoras Clean Code y Rust aplicadas, problemas corregidos.
3. Comandos ejecutados, riesgos pendientes y próximo paso.

**En revisión de código:**
Producir informes con un resumen de calidad, problemas detectados (diseño, Rust, Clean Code), riesgos, duplicidades, comandos de validación y un veredicto técnico (Correcto / Con mejoras menores / Necesita corrección).
