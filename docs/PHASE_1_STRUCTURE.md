# Fase 1 — Creación de la estructura real del proyecto MVP

## Objetivo
Crear la estructura base real del proyecto NebriPop sin implementar todavía funcionalidades de negocio, aislando la carpeta `experiments/`.

## Estructura Creada
- `backend/`: Inicializado proyecto Rust (Cargo) mínimo.
- `frontend/`: Inicializado proyecto Rust (Cargo) mínimo con índice HTML.
- `database/`: Creada carpeta para la BD SQLite.
- `docs/`: Actualizada la documentación general de arquitectura y entorno.
- `scripts/`: Creada carpeta para futuras utilidades.
- `.gitignore`: Actualizado para proteger archivos sensibles.
- `.env.example`: Creado sin secretos reales.

## Lo que no se ha implementado
- Login / Registro.
- Entidades reales (productos, usuarios, favoritos).
- Migraciones.
- Endpoints funcionales de Axum.
- Pantallas complejas de Leptos.

## Diferencia con experiments/
La carpeta `experiments/` contiene pruebas de concepto (PoC) aisladas. El código base en `backend/` y `frontend/` es la base limpia que se usará para el MVP final.

## Próxima Fase Recomendada
**Fase 2**: Implementar el esqueleto base de red (Endpoints vacíos de Axum y enrutamiento básico de Leptos).

## Resumen para Logs.md
Se ha preparado el entorno base, configurado `.gitignore` y el scaffolding de los proyectos en Rust, estableciendo reglas claras de arquitectura para el MVP de NebriPop.
