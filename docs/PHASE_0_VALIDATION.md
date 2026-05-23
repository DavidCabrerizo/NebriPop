# Fase 0 — Validación final antes de implementar el MVP

## 1. Objetivo de la fase
El objetivo principal de la Fase 0 es validar todas las decisiones finales del proyecto, revisar la organización de la documentación, auditar el estado del repositorio y verificar que todo el equipo de agentes de Inteligencia Artificial esté preparado antes de empezar a implementar código real del MVP de NebriPop. Esta es una fase exclusivamente de revisión y preparación organizativa.

## 2. Stack tecnológico validado
Se ha validado el siguiente stack tecnológico como definitivo para el MVP:
- **Frontend:** Rust + Leptos
- **Backend:** Rust + Axum
- **Base de datos:** SQLite
- **Control de versiones:** Git + GitHub
- **Coordinación:** Antigravity (Agent Manager)
- **Implementación técnica:** OpenCode

*Nota del Agent Manager:* Se ha detectado que `PRD.md` e `IMPLEMENTATION_PLAN.md` mencionan **PostgreSQL**. La decisión final validada es **SQLite** por su simplicidad para el MVP y evitar contenedores adicionales de base de datos en las primeras etapas.

## 3. Estado de las pruebas técnicas previas
Las pruebas técnicas previas (Calculadora backend, frontend con Leptos, FullStack, integración pre-MVP con SQLite) se han realizado exitosamente y se encuentran aisladas en la carpeta `experiments/`. Han cumplido su objetivo de demostrar la viabilidad del stack tecnológico y permitir a los agentes familiarizarse con la interoperabilidad entre Axum, Leptos y SQLite.

## 4. Estado de los agentes
Los agentes del sistema han sido revisados y validados.
- Sus perfiles, responsabilidades y limitaciones están correctamente documentados en `AGENTS.md`.
- Las skills (`clean-rust-skill`, `ui-ux-pro-max`, `rust-best-practices`, etc.) están integradas.
- La jerarquía subordinada al **Agent Manager / Product Manager Agent** está garantizada mediante las reglas (`RULES.md`).

## 5. Estado de la documentación
- Existen documentos clave: `PRD.md`, `IMPLEMENTATION_PLAN.md`, `AGENTS.md`, `AI_WORKFLOW.md`, `RULES.md` y `README.md`.
- `docs/Logs.md` se encuentra actualizado con todos los experimentos previos.
- Los prompts están correctamente almacenados en `docs/Prompts/`.
- Faltan por crear `docs/ARCHITECTURE.md` y `docs/DATABASE_SCHEMA.md`, los cuales deben ser redactados en la Fase 1 antes de codificar la estructura principal.

## 6. Estado del repositorio GitHub
Auditoría completada por el **GitHubAgent**:
- El repositorio de Git está correctamente inicializado.
- El origen (`origin`) apunta correctamente a `https://github.com/DavidCabrerizo/NebriPop`.
- El árbol de trabajo está limpio (`working tree clean`).
- El archivo `.gitignore` está correctamente configurado para proteger archivos sensibles (`.env`, `*.db`, `*.sqlite`, `target/`, etc.).

## 7. Estado del entorno de desarrollo
El entorno está validado por el **DevopsAgent**:
- Rust está configurado.
- Se ha validado la necesidad de instalar `trunk` y añadir el target `wasm32-unknown-unknown` para compilar Leptos en la Fase 1.
- Puertos recomendados aceptados: Backend (3000), Frontend (8081).
- Pendiente de creación el archivo `.env.example` en la Fase 1.

## 8. Riesgos detectados
- **Riesgo Documental (Medio):** Discrepancia entre la base de datos validada (SQLite) y la documentada en `PRD.md` / `IMPLEMENTATION_PLAN.md` (PostgreSQL).
- **Riesgo Arquitectónico (Medio):** Falta la definición detallada de la arquitectura (`docs/ARCHITECTURE.md`) y el modelo de datos formal (`docs/DATABASE_SCHEMA.md`).
- **Riesgo de Entorno (Bajo):** Ausencia de un archivo `.env.example` que guíe la configuración local segura de variables de entorno.

## 9. Recomendaciones antes de Fase 1
1. Actualizar `PRD.md` e `IMPLEMENTATION_PLAN.md` y `README.md` para reflejar el uso de **SQLite** como base de datos definitiva del MVP.
2. Autorizar al **RustArchitectAgent** como primera tarea de la Fase 1 para crear `docs/ARCHITECTURE.md` y plantear la separación de carpetas.
3. Definir formalmente `docs/DATABASE_SCHEMA.md`.
4. Crear `.env.example` con las variables de entorno esperadas antes de codificar la conexión a la base de datos.

## 10. Criterios de aceptación de Fase 0
- [x] PRD y Plan de Implementación revisados.
- [x] Stack tecnológico confirmado (Leptos + Axum + SQLite).
- [x] Pruebas técnicas aisladas en `experiments/`.
- [x] Agentes correctamente documentados.
- [x] Repositorio auditable, limpio y apuntando al remoto correcto.
- [x] `docs/Prompts/` actualizado.
- [x] Riesgos iniciales identificados.

## 11. Veredicto final
**Fase 0 aprobada con ajustes menores.**
Se autoriza comenzar la Fase 1, priorizando la resolución de las discrepancias documentales (SQLite vs PostgreSQL) y la creación del esquema arquitectónico antes de implementar código funcional.
