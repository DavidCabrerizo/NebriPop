# Decisiones de Arquitectura y Backend para la Base de Datos

Documento generado por **RustArchitectAgent** y **RustBackend**.

## 1. Decisiones Arquitectónicas (RustArchitectAgent)
- **Adecuación al MVP**: El diseño encapsula perfectamente el core de un marketplace tipo Wallapop. Tres tablas (users, categories, products) son suficientes para el flujo principal de publicar y buscar productos.
- **Preparado para escalar**: La inclusión de `product_images`, `favorites` y `messages` demuestra visión a medio plazo sin introducir sobreingeniería actual, ya que no estamos obligados a implementar sus endpoints en la Fase 3.
- **Separación de responsabilidades**: La carpeta `database/` se mantiene aislada y servirá de fuente de verdad. El backend sólo interactuará a través de un pool de conexiones SQLite (por ejemplo, con `sqlx`).

## 2. Decisiones de Implementación Backend (RustBackend)
- **Tipos de Datos y Mapeo en Rust**:
    - Los `TEXT` de SQLite mapearán bien a `String` en Rust.
    - Los `INTEGER PRIMARY KEY` mapearán a `i64` o `i32`.
    - `REAL` mapeará a `f64`.
    - Los tipos de fechas (almacenados como `TEXT`) podrán parsearse con `chrono` a `DateTime<Utc>` en las structuras de Axum.
- **Restricciones CHECK**: Los enumerados (`condition`, `status`) se manejan como validaciones CHECK a nivel DB. En Rust será recomendable crear `enum` y derivar `sqlx::Type` (si se usa sqlx) o manejar strings simples en la primera iteración.
- **Conexión recomendada para Fase 3**: Sugerimos el uso de `sqlx` con la flag `sqlite` para que Axum se conecte asíncronamente y pueda validar las consultas en tiempo de compilación frente al `schema.sql`.
