# Fase 2 — Diseño de la base de datos SQLite real del MVP

## 1. Objetivo de la fase
Diseñar y preparar la base de datos SQLite real del MVP, definiendo tablas principales, campos, relaciones, restricciones, datos iniciales y documentación, sin implementar funcionalidades backend/frontend.

## 2. Enfoque tipo Wallapop
El modelo soporta un marketplace generalista para la compra y venta de artículos de segunda mano entre usuarios, categorizados y localizados.

## 3. Tablas diseñadas
- `users`: Usuarios de la plataforma.
- `categories`: Categorías de clasificación.
- `products`: Anuncios de artículos publicados.
- `product_images`: Imágenes múltiples por anuncio (preparado).
- `favorites`: Lista de deseos de los usuarios (preparado).
- `messages`: Comunicación básica entre usuarios (preparado).

## 4. Relación entre tablas
- `products` -> pertenece a `users` (vendedor) y `categories`.
- `product_images` -> pertenece a `products`.
- `favorites` -> asocia `users` y `products`.
- `messages` -> asocia `users` (emisor), `users` (receptor) y `products`.

## 5. Archivos creados
- `database/schema.sql`
- `database/seeds.sql`
- `database/README.md`
- `docs/DATABASE_SCHEMA.md`
- `docs/DATABASE_DECISIONS.md`
- `docs/QA_PHASE_2_DATABASE.md`
- `docs/SECURITY_PHASE_2_DATABASE.md`
- `docs/REVIEW_PHASE_2.md`
- `docs/Prompts/PromptFase2DatabaseMVP.txt`

## 6. Decisiones tomadas
Separar físicamente la lógica DB en la carpeta `database/`. Mantener el esquema minimalista pero listo para escalar con índices y restricciones.

## 7. Restricciones aplicadas
- Unicidad en emails y categorías.
- Precios positivos.
- Estados de producto (available, reserved, sold).
- Condición de producto limitada a valores específicos.

## 8. Categorías iniciales
Electrónica, Hogar, Moda, Vehículos, Deporte, Libros, Videojuegos, Muebles, Herramientas, Jardín, Bebés y niños, Mascotas, Coleccionismo, Otros.

## 9. Seeds iniciales
8 productos de ejemplo asignados a un usuario dummy.

## 10. Riesgos detectados
La falta de foreign keys habilitadas por defecto en SQLite. **Mitigación**: Añadido aviso explícito para que el backend ejecute `PRAGMA foreign_keys = ON;` al conectar.

## 11. Pruebas recomendadas
Ejecutar scripts SQL localmente con `sqlite3` y verificar el diagrama relacional.

## 12. Próxima fase recomendada
**Fase 3:** Implementación inicial del Backend Axum, configuración de rutas básicas y conexión real a la base de datos SQLite para leer categorías o productos.
