# Reporte de Seguridad Final MVP - NebriPop

## Resumen de Auditoría
Se ha realizado una revisión final de seguridad sobre el código base del MVP de NebriPop, tanto en el backend (Rust con Axum) como en el frontend (Rust con Leptos). A continuación, se detallan los hallazgos en las diferentes categorías evaluadas, junto con las recomendaciones futuras y limitaciones del alcance MVP.

## 1. Exposición de Secretos y Credenciales
- **Contraseñas:** No se han expuesto contraseñas en texto plano. Se utiliza `bcrypt` para hacer el hashing de las contraseñas antes de almacenarlas en la base de datos y se utiliza la directiva `#[serde(skip_serializing)]` en el modelo `User` para garantizar que el `password_hash` nunca se exponga en las respuestas JSON.
- **Variables de Entorno y Secretos:** El archivo `.env` está correctamente excluido en el `.gitignore`. Solamente se expone `.env.example` de forma segura. Tras la búsqueda en todo el código base, no se encontraron tokens (API keys), JWT hardcodeados ni secretos expuestos.

## 2. Prevención de Vulnerabilidades Clásicas
- **SQL Injection:** El backend emplea el crate `sqlx` con variables de enlace seguras (`.bind()`) y el `QueryBuilder` con `.push_bind()`. No existen construcciones de SQL dinámico basadas en concatenación de strings no confiables. Por lo tanto, el riesgo de SQL Injection se considera **Mitigado**.
- **Path Traversal:** El sistema de subida de imágenes para productos ignora el nombre de archivo enviado por el usuario, evitando saltos de directorio (por ejemplo, `../../`). Renombra automáticamente el archivo destino a un formato controlado (`imagen_{count}.{ext}`) limitando además la extensión a archivos de imagen (`jpg`, `jpeg`, `png`) y controlando el tamaño máximo. El riesgo de Path Traversal se considera **Mitigado**.
- **Cross-Site Scripting (XSS):** El frontend está desarrollado con `Leptos` (Rust), que por diseño escapa de forma automática la interpolación de variables insertadas en la macro `view!`, convirtiéndolas en Text Nodes seguros en el DOM en lugar de inyectar código HTML raw. La protección básica contra XSS está **Implementada**.

## 3. Configuración CORS
El backend está actualmente configurado utilizando el crate `tower_http::cors` con la siguiente directiva:
```rust
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PUT, Method::DELETE, Method::PATCH])
    .allow_headers(Any);
```
- **Veredicto:** Funcional para entornos de desarrollo y para este MVP. Sin embargo, en un entorno de producción, permitir `allow_origin(Any)` expone la API a peticiones cruzadas desde cualquier dominio malicioso.

## 4. Limitaciones de Seguridad del MVP (Autenticación y Autorización)
Dado el alcance acordado para el MVP de la aplicación, existen importantes limitaciones y riesgos asumidos en cuanto a la autenticación:
- **Ausencia de Sesiones Reales (No JWT / No Cookies seguras):** Tras el inicio de sesión (`/login`), el backend valida la contraseña y devuelve un objeto `User`, pero no proporciona un token de sesión (JWT) firmado, ni establece una sesión de backend. El frontend simplemente mantiene el `user_id` almacenado en memoria o LocalStorage asumiendo que el usuario está "autenticado".
- **Insecure Direct Object Reference (IDOR):** Como consecuencia de no usar tokens, los endpoints protegidos, como la creación, edición o borrado de productos (`UpdateProductDto`, `CreateProductDto`), confían ciegamente en el campo `user_id` enviado en el cuerpo de la petición.
    - *Riesgo:* Un atacante podría interceptar las llamadas API y modificar el `user_id` por el de otra persona para publicar, borrar o modificar artículos en nombre de otros usuarios.
- **Autorización Inexistente o Basada en el Cliente:** La seguridad actual descansa principalmente en el cliente (Ocultar botones si el ID no coincide). Esto es fundamentalmente inseguro en un entorno expuesto.

## Veredicto Final
El MVP cumple con los **requisitos mínimos de seguridad locales y preventivos** exigidos para la entrega de esta iteración (SQL injection controlada, sin secretos subidos, path traversal contenido). Sin embargo, carece de mecanismos fundamentales de sesión y autorización robusta. 

**Estado de Auditoría:** APROBADA (Con limitaciones explícitas para el MVP).

## Recomendaciones Futuras (Post-MVP)
1. **Implementar JWT (JSON Web Tokens):** Cambiar el proceso de login para devolver un JWT firmado (RS256/HS256) y configurar middleware en Axum para exigir `Authorization: Bearer <token>` en rutas protegidas.
2. **Validar Autorización en Backend:** Eliminar la dependencia del `user_id` enviado desde el Frontend. El `user_id` que realiza la acción debe extraerse directamente del claim del JWT verificado.
3. **Restringir CORS:** Cambiar `allow_origin(Any)` por los dominios exactos del frontend cuando se despliegue a producción.
4. **Rate Limiting:** Implementar un límite de llamadas, especialmente en el endpoint de subida de imágenes, para evitar ataques de denegación de servicio (DoS) llenando el disco.
