# QA Fase 9: Mensajería Básica

## Pruebas Realizadas

### Backend
- `[x]` **POST /messages**: Valida correctamente campos vacíos (ej. `content`), producto inexistente, y que un usuario no se hable a sí mismo. Guarda el mensaje en la base de datos `messages` local de SQLite.
- `[x]` **GET /users/:id/messages** y **/sent**: Retornan correctamente la lista de mensajes en los que el usuario es destinatario o remitente.
- `[x]` **GET /products/:id/messages?user_id=X**: Devuelve toda la conversación ordenada cronológicamente asociada a un producto en particular donde el usuario haya intervenido.
- `[x]` **PATCH /messages/:id/read**: Permite a un receptor marcar su mensaje como leído.

### Frontend
- `[x]` **Botón Contactar**: Aparece en `product_detail` si hay una sesión activa y el usuario NO es el dueño.
- `[x]` **Página de Mensajes**: Accesible desde el Perfil del usuario, agrupa todas las conversaciones según `product_id`.
- `[x]` **Página de Conversación**: Renderiza en forma de chat el hilo de mensajes para un producto. Muestra quién envía el mensaje, lo alinea, y permite mandar nuevos mensajes.
- `[x]` **Manejo de Errores**: Todos los errores de "sin sesión" o "mensaje vacío" se muestran mediante el componente `<ErrorMessage />`.

### Base de Datos
- `[x]` Los mensajes se persisten limpiamente en la tabla `messages` ya existente desde la fase 2.
- `[x]` Cero uso de `.json`. SQLite se mantiene como fuente única de la verdad.

**Resultado de QA:** Aprobado.
