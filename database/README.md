# NebriPop Database (SQLite)

Esta carpeta contiene la base de datos embebida (SQLite), sus esquemas y datos iniciales para el MVP de **NebriPop**.

## 📌 Estado Actual
**MVP Finalizado (Fase 10).** La base de datos es totalmente funcional y contiene las siguientes tablas modeladas según un entorno de e-commerce real:
- `users`: Usuarios y contraseñas (bcrypt hashes).
- `categories`: Categorías fijas de productos académicos y generales.
- `products`: Anuncios de compraventa, indexados por categorías, precios y ubicaciones.
- `product_images`: URLs relativas de imágenes subidas localmente (On Delete Cascade).
- `favorites`: Relación Mucho-a-Mucho de favoritos guardados.
- `messages`: Tabla asíncrona de conversaciones y bandejas de entrada para la negociación de compra.

## 📂 Archivos
- `schema.sql`: Estructura transaccional, llaves foráneas y constraints relacionales.
- `seeds.sql`: Datos semilla de prueba (usuarios mock y productos pre-cargados).
- `nebripop.db`: Base de datos SQLite real. Actualmente configurada para subir a GitHub por propósitos de evaluación académica (modificado en Fase 8/9).

## ⚙️ Cómo inicializar la base de datos

Si la base de datos no existe o deseas resetearla al estado inicial:

**En Linux / macOS / Git Bash:**
```bash
sqlite3 database/nebripop.db < database/schema.sql
sqlite3 database/nebripop.db < database/seeds.sql
```

**En Windows (PowerShell):**
```powershell
Get-Content database\schema.sql | sqlite3 database\nebripop.db
Get-Content database\seeds.sql | sqlite3 database\nebripop.db
```

## 🛠️ Reglas Importantes de Interacción con Rust (Axum/SQLx)
1. **Protección:** Siempre utilizamos consultas parametrizadas (ej: `bind()`) previniendo Inyecciones SQL.
2. **Concurrencia:** SQLite está conectada utilizando un pool de conexiones con `journal_mode=WAL` configurado vía código en `main.rs`.
3. **Casos Especiales:** El borrado de productos gatilla un ON DELETE CASCADE limpiando también `product_images` y `favorites`.
