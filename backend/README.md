# NebriPop Backend

Este es el módulo del servidor (API REST) para la plataforma **NebriPop**. Ha sido desarrollado íntegramente en Rust.

## 🚀 Tecnologías
- **Framework Web:** [Axum](https://github.com/tokio-rs/axum)
- **Runtime Asíncrono:** [Tokio](https://tokio.rs/)
- **Capa de Persistencia:** [SQLx](https://github.com/launchbadge/sqlx) con SQLite
- **Autenticación:** JWT + bcrypt
- **Arquitectura:** N-Capas (Routes, Handlers, Services, Repositories).

## 📌 Estado Actual
**MVP Finalizado (Fase 10).** El backend es totalmente funcional, manejando:
- Autenticación segura.
- Gestión de productos con subida local de imágenes multipart.
- Filtros complejos construidos dinámicamente (`QueryBuilder`).
- Mensajería asíncrona entre usuarios.
- Protección general anti inyección SQL (SQLi).

## ⚙️ Uso
Asegúrate de estar en el directorio `backend/` y de tener la base de datos `nebripop.db` (ubicada en `database/`) configurada en tu archivo `.env`.

```bash
# Iniciar servidor en http://127.0.0.1:3000
cargo run
```
