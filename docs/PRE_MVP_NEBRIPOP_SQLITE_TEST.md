# Test Pre-MVP: NebriPop con SQLite, Axum y Leptos

## Objetivo de la prueba
Construir una mini aplicación pre-MVP completamente aislada que unifique el frontend en Leptos con un backend en Axum y demuestre la persistencia real de datos en una base de datos SQLite embebida, sentando las bases tecnológicas definitivas antes del desarrollo del MVP.

## Por qué se hace antes del MVP
Para certificar que el stack elegido es funcional de extremo a extremo, verificando que la gestión concurrente de SQLite en Tokio y la serialización JSON operan con un rendimiento y simplicidad aceptables bajo un esquema de datos tipo "Wallapop".

## Stack Utilizado
- **Frontend**: Rust (Leptos CSR), `reqwest`.
- **Backend**: Rust (Axum), `tokio` para concurrencia asíncrona, `tower-http` para CORS.
- **Base de Datos**: SQLite (mediante la crate `rusqlite` y su feature `bundled`).

## Agentes Participantes
1. **Agent Manager**: Orquestación y control del Sandbox.
2. **UXDesigner**: Definición de UI sencilla.
3. **RustArchitectAgent**: Planificación de la división en tres carpetas.
4. **RustBackend**: Desarrollo del backend, conexión con DB e inicialización de tablas en base al schema.
5. **RustFrontend**: Consumo de la API con WASM, gestión de estados con Signals.
6. **DatabaseAgent**: Creación de esquema `schema.sql`.
7. **QAAgent**: Validación de casos de prueba.
8. **SequrityAgent**: Documentación de prevención de riesgos (SQL Injection).
9. **DevopsAgent**: Documentación de ejecución concurrente local.
10. **DocumentationAgent** / **ReviewerAgent**: Informes de validación.

## Funcionalidad Implementada
- **Estructura de carpetas**: `backend`, `frontend`, `database`.
- Creación e inserción de 2 productos "dummy" iniciales si la DB está vacía.
- Endpoint `GET /products` que retorna el array de productos.
- Endpoint `POST /products` que inyecta en SQL un producto nuevo tras superar validaciones.
- Reflejo directo e inmediato en el UI de Reactividad Web de Leptos.

## Problemas Detectados y Conclusiones
No se detectaron problemas críticos. La ejecución es rápida y funcional. La aplicación persiste los datos más allá del ciclo de vida del backend, demostrando que SQLite es adecuado.

## Recomendaciones para el MVP real
- Migrar de `rusqlite` a `sqlx` si se requieren migraciones más avanzadas y comprobaciones en tiempo de compilación.
- Implementar paginación en `GET /products`.
- Añadir un JWT o sistema de sesiones.

---

### Resumen preparado para registrar en docs/Logs.md
- **Fecha**: 22 de mayo de 2026.
- **Agente responsable**: Agent Manager.
- **Acción realizada**: Creación de aplicación Pre-MVP completa en `experiments/pre_mvp_nebripop_sqlite/`.
- **Archivos afectados**: `schema.sql`, código fuente frontend/backend y documentos (QA, DevOps, Security, Test, Review).
- **Motivo del cambio**: Comprobar persistencia real con SQLite antes de codificar el MVP principal.
- **Resumen**: App pre-MVP completamente aislada pero 100% funcional. Listado, creación, persistencia y visualización web interconectados y asegurados con consultas parametrizadas.
- **Próximo paso recomendado**: Arrancar oficialmente la arquitectura real del MVP de NebriPop (Fase 1).
