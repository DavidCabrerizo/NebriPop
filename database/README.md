# NebriPop Database (MVP)

Esta carpeta contiene el diseño de la base de datos SQLite real para el MVP de NebriPop.

## Archivos

- `schema.sql`: Define las tablas, relaciones (Foreign Keys) y restricciones (CHECK) para el modelo de datos inicial (Fase 2).
- `seeds.sql`: Contiene los datos iniciales obligatorios (categorías) y algunos datos de ejemplo (usuarios y productos) para facilitar el desarrollo.
- `nebripop.db`: Archivo SQLite generado al importar los esquemas y seeds. **No debe subirse a control de versiones si contiene datos de producción o sensibles**.

## Cómo inicializar la base de datos

Si la base de datos no existe o deseas recrearla desde cero, puedes usar los siguientes comandos desde la raíz del proyecto (requiere `sqlite3`):

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

## Comandos Útiles para Interactuar (CLI)

Puedes abrir la consola interactiva de la base de datos con:
```bash
sqlite3 database/nebripop.db
```

Dentro de la consola de `sqlite3`, prueba estos comandos:
- `.tables` : Lista todas las tablas.
- `.schema products` : Muestra cómo se creó la tabla de productos.
- `.mode box` y `.header on` : Activa una vista de tablas mucho más legible.
- `SELECT * FROM categories;` : Verifica las 14 categorías iniciales.
- `SELECT * FROM products;` : Comprueba los datos semilla cargados.
- `.quit` : Sale de la interfaz.

## Reglas importantes
1. **Seguridad**: No guardar contraseñas en texto plano. **Siempre** usar consultas parametrizadas en Rust (`?1`, `:name`) para evitar SQL Injection.
2. **Integridad**: Ejecutar `PRAGMA foreign_keys = ON;` al conectar con SQLite desde Rust para garantizar las restricciones.
3. **Rendimiento**: Se recomienda activar el modo WAL para mejorar la concurrencia en la app: `PRAGMA journal_mode = WAL;`.
