# Fase 6: Sistema Básico de Usuarios (MVP)

Esta fase implementa la gestión básica de usuarios en NebriPop, uniendo finalmente los productos publicados con los usuarios de la plataforma y proporcionando mecanismos de autenticación.

## Backend (Axum + SQLite)

### Endpoints implementados
- `POST /auth/register`: Recibe `name`, `email` y `password` (junto a datos opcionales como `phone` y `location`). Encripta el `password` usando `bcrypt` y guarda el usuario.
- `POST /auth/login`: Recibe `email` y `password`, comprueba el hash usando `bcrypt` y devuelve el objeto `User` al autenticarse correctamente.
- `GET /users/:id`: Devuelve el perfil público de un usuario por su `id`.
- `GET /users/:id/products`: Devuelve la lista de productos publicados por ese usuario.

### Modelos y Repositorios
- Creada estructura `User` en `src/models.rs`.
- Creado `user_repository.rs` para realizar operaciones CRUD en la tabla `users` mediante `sqlx`.
- Añadido el método `find_by_user_id` en `product_repository.rs`.
- Evitada la serialización de `password_hash` hacia el frontend mediante `#[serde(skip_serializing)]`.

## Frontend (Leptos)

### Componentes y Páginas Creadas
- **Página de Registro (`/register`)**: Formulario para crear una nueva cuenta.
- **Página de Login (`/login`)**: Formulario de autenticación que al tener éxito guarda `user_id` y `user_name` en `localStorage` (solución temporal para MVP).
- **Página de Perfil (`/profile`)**: Muestra los datos del usuario logueado extrayendo su ID de `localStorage`.
- **Productos del Usuario (`/users/:id/products`)**: Muestra el catálogo de productos asociados a un ID de usuario concreto.

### Modificaciones en Productos
- La publicación de productos en `CreateProduct` ahora exige estar logueado (comprueba `localStorage`).
- Envía el `user_id` correcto al backend, asociando el producto al usuario real en vez de usar un ID hardcodeado.

## Resumen de Decisiones
Se ha optado por `localStorage` como mecanismo de persistencia temporal de la sesión para evitar añadir la complejidad de JWT e interceptores al MVP actual, priorizando la simplicidad requerida en las instrucciones iniciales.

## Próximos pasos recomendados
Con la fase 6 finalizada, los productos están ahora ligados a usuarios y la subida de imágenes de la fase 5B está activa. El MVP inicial planteado para NebriPop está fundamentalmente **completo y funcional**. En futuras fases o versiones post-MVP se deberá reemplazar el uso de `localStorage` por `JWT + HttpOnly Cookies` y añadir la funcionalidad de mensajería/chat si es necesario.
