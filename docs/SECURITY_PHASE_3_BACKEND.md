# Riesgos de Seguridad Fase 3 Backend

## Revisiones Realizadas

- [x] **Inyección SQL**: Las consultas SQLite utilizan sentencias parametrizadas nativas de SQLx (ej. `sqlx::query!`, `sqlx::query_as!`), lo que previene la concatenación de strings y evita inyecciones SQL.
- [x] **Filtrado de información**: Los errores internos de base de datos se capturan en `errors.rs` y se devuelven al cliente como mensajes genéricos (`"Error interno del servidor"`), evitando la exposición de detalles de la infraestructura.
- [x] **Credenciales**: No se han subido credenciales reales. El archivo `.env.example` contiene valores de ejemplo inofensivos.
- [x] **CORS**: Se ha configurado CORS usando `tower-http` en `main.rs`. Aunque actualmente usa `Any` para facilitar el desarrollo local inicial, se ha dejado un comentario para limitarlo a `http://127.0.0.1:8081` en producción.
- [x] **Validación de Datos**: `POST /products` valida explícitamente los campos obligatorios y sus rangos (precios negativos, cadenas vacías, valores de enumeración) antes de llegar a la base de datos.
- [x] **Autenticación**: Dado que el login no está implementado (fuera del alcance de esta fase), no se han introducido mecanismos de autenticación falsos o inseguros, y el campo `user_id` de productos asume el comportamiento de MVP especificado. No se guardan contraseñas en esta fase.

## Recomendaciones para Siguientes Fases
- Limitar explícitamente los orígenes CORS.
- Implementar validación de JWT segura en la Fase correspondiente.
