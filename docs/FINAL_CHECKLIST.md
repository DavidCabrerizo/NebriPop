# Checklist Final de Entrega - MVP NebriPop

Este documento detalla la lista de verificación requerida por la Fase 10 antes de considerar cerrado el MVP.

## Backend (Axum + SQLite)
- [x] El backend compila sin errores (`cargo build`).
- [x] El servidor levanta exitosamente (`cargo run` en puerto 3000).
- [x] Endpoints esenciales funcionales y testeados (`/health`, `/users`, `/products`, `/categories`).
- [x] La creación y consulta de Mensajes y Favoritos están operativas.
- [x] Persistencia de datos gestionada 100% en SQLite (`nebripop.db`).
- [x] No hay archivos locales de persistencia basados en `.json`.

## Frontend (Leptos + Trunk)
- [x] Compila a Wasm correctamente.
- [x] Arranca con Trunk (`trunk serve` en puerto 8081).
- [x] Vista principal (Home) renderiza productos.
- [x] Vistas de Registro y Login funcionan y muestran feedback de error.
- [x] Vista de Detalle de Producto muestra la información completa.
- [x] Formulario de Creación de Producto funcional y validado.
- [x] Sistema de búsqueda y filtrado de productos integrado y estable.
- [x] Vistas de perfil, mensajería y favoritos renderizan correctamente los datos.
- [x] No existen enlaces rotos, ni vistas en blanco ("pantallas rotas").

## Base de Datos (SQLite)
- [x] Archivo `schema.sql` completo (users, products, categories, messages, favorites, etc.).
- [x] Archivo `seeds.sql` operativo y libre de información personal real.
- [x] Relaciones e Integridad Referencial confirmadas.
- [x] Índices creados para optimización de consultas.

## Entrega y Repositorio
- [x] `.env.example` existe y no incluye tokens o contraseñas reales.
- [x] `.gitignore` configurado correctamente (ignora `.env`, `target/`, `nebripop.db`, `uploads/`).
- [x] Documentación técnica y de usuario estructurada en `docs/`.
- [x] README.md en raíz, en `backend/`, `frontend/` y `database/` listos.
- [x] Comandos de ejecución locales claramente documentados para evaluación de terceros.

**Veredicto de Verificación:** Todas las casillas completadas y validadas por QAAgent.
**Checklist Aprobado** ✅
