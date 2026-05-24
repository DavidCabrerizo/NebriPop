# Arquitectura del MVP de NebriPop

Este documento define la arquitectura general del MVP de NebriPop.

## Estructura General
El proyecto sigue una separación clara entre el frontend y el backend, organizados en un único repositorio monolítico para facilitar el desarrollo inicial (Fase 1).

### Directorios principales:
- `backend/`: Contendrá la API REST (Rust + Axum) que sirve los datos y gestiona la lógica de negocio.
- `frontend/`: Contendrá la interfaz web del usuario (Rust + Leptos, compilado a WebAssembly).
- `database/`: Almacena la base de datos local SQLite y scripts de migración.
- `docs/`: Documentación del proyecto (arquitectura, setup, progreso por fases).
- `scripts/`: Herramientas auxiliares.
- `experiments/`: Código descartable y pruebas previas (aislado del MVP real).

## Decisiones Técnicas (MVP)
1. **Comunicación Backend-Frontend**: El frontend se comunicará con el backend mediante peticiones HTTP (REST).
2. **Base de Datos**: SQLite, elegida por su simplicidad y porque encaja en un MVP académico o local sin necesidad de infraestructura pesada.
3. **Manejo de estados en UI**: Leptos utiliza reactividad basada en señales.

## Reglas (Fase 1)
- Evitar sobreingeniería.
- Aislamiento estricto de la carpeta `experiments/`.
- No implementar lógica de negocio todavía. Preparar el entorno.
