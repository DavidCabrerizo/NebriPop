# Fase 9 — Mensajería básica entre usuarios

## 1. Objetivo de la fase
Implementar una mensajería básica que permita a un comprador ponerse en contacto con el vendedor de un producto para acordar la venta, y disponer de una bandeja de entrada para consultar las conversaciones.

## 2. Plan de implementación aprobado
Se realizó el plan en `implementation_plan.md` proponiendo una arquitectura REST (sin WebSockets) e integrando componentes de UI reactivos con Leptos. Se usó la tabla `messages` ya existente en la BD.

## 3. Funcionamiento de la mensajería
- Un usuario logueado abre un producto ajeno y pulsa **Contactar**.
- Esto le abre la pantalla `/products/:id/conversation` donde ve el hilo previo o envía el primer mensaje.
- El vendedor recibe el mensaje (se guarda en SQLite). 
- El usuario puede ir a **Perfil > Mis Mensajes** (`/messages`) para ver todas las conversaciones abiertas.

## 4. Endpoints implementados
- `POST /messages`
- `GET /users/:id/messages`
- `GET /users/:id/messages/sent`
- `GET /products/:product_id/messages`
- `PATCH /messages/:id/read`

## 5. Cambios en backend
- Creación de DTOs en `message_dto.rs`.
- Lógica de persistencia en `message_repository.rs`.
- Controladores en `messages_handler.rs`.
- Rutas agrupadas en `routes/messages.rs`.

## 6. Cambios en frontend
- Creación de `models/message.rs`.
- Cliente HTTP en `api/messages_api.rs`.
- Páginas creadas: `pages/messages.rs` (buzón) y `pages/conversation.rs` (hilo de chat).
- Integración en el `product_detail.rs` (botón Contactar) y `profile.rs` (botón Mis Mensajes).
- Enrutado actualizado en `app.rs`.

## 7. Cambios en SQLite
Ninguno requerido. La tabla `messages` había sido correctamente planificada en la Fase 2.

## 8. Validaciones aplicadas
- Límite de caracteres en mensajes.
- No es posible enviarse mensajes a uno mismo.
- Solo se puede escribir si existe el producto.
- En frontend se ocultan botones de contacto si es tu propio producto o no estás logueado.

## 9. Seguridad
Revisión completada en `SECURITY_PHASE_9_MESSAGES.md`. (Persiste la necesidad de autenticación real basada en JWT para la Fase 10).

## 10. Pruebas realizadas
Revisión completada en `QA_PHASE_9_MESSAGES.md`.

## 11. Limitaciones del MVP
No hay notificaciones en tiempo real, no hay cifrado E2E, no se permite envío de imágenes por el chat y no hay comprobación real de identidades (falta JWT). Todo esto se asume por tratarse de un MVP básico.

## 12. Resultado final
Fase completada con éxito.

## 13. Próxima fase recomendada
**Fase 10 — Autenticación robusta (JWT)**: Ahora que la aplicación tiene todas sus ramas lógicas funcionando, es crítico securizar el endpoint inyectando JWT, eliminando la vulnerabilidad de spoofing basada en `localStorage`.
