# Arquitectura de Autenticación Básica (Fase 6)

Este documento ha sido redactado por el **RustArchitectAgent** siguiendo las instrucciones del **Agent Manager**. Define la arquitectura y reglas técnicas para implementar el sistema básico de usuarios de NebriPop (Fase 6).

## 1. Visión General
Para el MVP, la arquitectura prioriza la simplicidad y la velocidad de implementación sin comprometer la seguridad fundamental.
- **Base de Datos:** Se usará estrictamente `SQLite` mediante SQLx.
- **Backend:** Axum proveerá endpoints REST.
- **Frontend:** Leptos manejará la UI y un estado global básico.

## 2. Flujo de Registro
1. **Frontend:** El usuario completa `/register` (name, email, password, etc.).
2. **Backend (`POST /auth/register`):**
   - Valida datos de entrada (email único, password > 6 chars).
   - Genera un hash del password usando `argon2`.
   - Inserta el registro en la tabla `users`.
   - Devuelve un DTO con los datos del usuario, omitiendo SIEMPRE el `password_hash`.
3. **Frontend:** Muestra éxito y redirige a `/login`.

## 3. Flujo de Login
1. **Frontend:** El usuario completa `/login` (email, password).
2. **Backend (`POST /auth/login`):**
   - Busca al usuario por email en `users`.
   - Comprueba que el password coincide con `password_hash` mediante `argon2`.
   - Devuelve un mensaje de éxito junto con los datos básicos del usuario.
3. **Frontend:**
   - Guarda temporalmente `user_id`, `name`, y `email` en `LocalStorage` o en un Contexto global de Leptos.
   - **Nota temporal MVP:** No se implementará JWT estricto en esta fase por directriz del Manager. La presencia del `user_id` en el estado local indica "usuario conectado".

## 4. Perfil y Asociación de Productos
- **Perfil:** `GET /users/:id` devolverá los datos de perfil.
- **Productos del Usuario:** `GET /users/:id/products` leerá la tabla `products` usando `WHERE user_id = :id`.
- **Publicar Producto:** El frontend enviará el `user_id` en el JSON de creación (`POST /products`). El backend debe aceptarlo e insertarlo.

## 5. Reglas Estrictas
- **NO** se usan archivos `.json` para persistencia bajo ninguna circunstancia.
- **NO** se devuelven contraseñas en plano ni hashes a través de la API.
- Todo agente debe mantener el código lo más simple posible.
