# NebriPop Database (MVP)

Esta carpeta contiene el diseño de la base de datos SQLite real para el MVP de NebriPop.

## Archivos

- `schema.sql`: Define las tablas, relaciones (Foreign Keys) y restricciones (CHECK) para el modelo de datos inicial (Fase 2).
- `seeds.sql`: Contiene los datos iniciales obligatorios (categorías) y algunos datos de ejemplo (usuarios y productos) para facilitar el desarrollo.
- `nebripop.db`: (Opcional) Archivo SQLite generado al importar los esquemas y seeds. **No debe subirse a control de versiones si contiene datos de producción o sensibles**.

## Cómo inicializar la base de datos

Desde la raíz del proyecto, puedes usar los siguientes comandos si tienes `sqlite3` instalado:

```bash
sqlite3 database/nebripop.db < database/schema.sql
sqlite3 database/nebripop.db < database/seeds.sql
```

## Reglas importantes
1. No guardar contraseñas en texto plano en la tabla `users`.
2. Ejecutar `PRAGMA foreign_keys = ON;` al conectar con SQLite desde Rust (Axum) para que las restricciones de clave foránea funcionen correctamente.
