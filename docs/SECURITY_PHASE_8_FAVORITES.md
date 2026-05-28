# Análisis de Seguridad Fase 8: Favoritos

## Revisiones de Seguridad
1. **Validación de Entradas**: Se valida `user_id` y `product_id`. La FK de SQLite previene guardar favoritos sobre registros huérfanos o inexistentes.
2. **Consultas Parametrizadas**: Todas las consultas SQL (`INSERT`, `DELETE`, `SELECT`, `EXISTS`) usan la sintaxis `.bind()` de `sqlx`, previniendo Inyecciones SQL (SQLi).
3. **Control de Accesos (Limitación MVP)**:
   - *Riesgo documentado*: Actualmente el MVP usa el `user_id` proveniente del cliente (ej. `localStorage`) sin validación de token JWT. Esto permite teóricamente que un usuario envíe un `user_id` manipulado al backend.
   - *Mitigación actual*: El endpoint DELETE exige comprobar que el registro a borrar pertenece a dicho `user_id`, impidiendo borrados indiscriminados (pero no falsificación de identidad).
   - *Propuesta Futura*: Migrar a autenticación por tokens (JWT o Sessions) donde el `user_id` se extraiga del token confiable en el backend, ignorando el enviado por el cliente.
4. **Almacenamiento Seguro**: Se utilizan las tablas SQLite preparadas para ello, prohibiendo cualquier uso de `.json`.
5. **Fuga de Datos**: El endpoint de verificación (`/favorites/check`) y el de listado (`/favorites`) exponen únicamente productos. No se retornan hashes de contraseñas ni datos sensibles adicionales.

**Veredicto de SequrityAgent**: Fase 8 segura dentro del alcance del MVP. Se requiere priorizar Auth real para fases futuras.
