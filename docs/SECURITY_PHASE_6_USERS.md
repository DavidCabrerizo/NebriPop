# Revisión de Seguridad: Fase 6 — Sistema Básico de Usuarios

## Resumen de la Auditoría
En la implementación de la Fase 6 se ha evaluado la seguridad del flujo de usuarios, prestando especial atención a que los datos sensibles se gestionen de manera segura.

## Puntos Verificados

1. **Almacenamiento de Contraseñas:**
   - **Mecanismo:** Se utiliza `bcrypt` (hash robusto) en `RustBackend`.
   - **Estado:** ✅ Las contraseñas en texto plano no se guardan bajo ninguna circunstancia.

2. **Respuestas de la API:**
   - **Validación:** El campo `password_hash` está excluido de la serialización JSON globalmente usando `#[serde(skip_serializing)]` en el modelo `User`.
   - **Estado:** ✅ Nunca se expone el hash al cliente.

3. **Persistencia de Sesión (MVP Temporal):**
   - **Mecanismo:** Dado que en esta fase no se implementa JWT ni mecanismos robustos por requerimiento de alcance (MVP simple), se utiliza `localStorage` para guardar el ID de usuario.
   - **Riesgo Identificado:** Un atacante con acceso XSS podría obtener o manipular el `user_id`.
   - **Mitigación Temporal:** Se acepta el riesgo debido al carácter educativo/MVP, pero **no se guardan contraseñas en localStorage**, solo un identificador y el nombre.

4. **Inyección SQL:**
   - **Mecanismo:** El uso de `sqlx` con consultas parametrizadas previene por completo inyecciones SQL en la autenticación.
   - **Estado:** ✅ Validado.

5. **Exposición de Archivos:**
   - **Validación:** Se comprobó que no existen archivos `.json` que guarden credenciales, contraseñas o sesiones en texto plano o cualquier formato inseguro.

## Recomendaciones para Fase 7 o Futuro
- Implementar **JWT (JSON Web Tokens)** reales con `HttpOnly cookies` en lugar de `localStorage`.
- Implementar validación estricta del formato del correo usando regex.
- Implementar rate-limiting en los endpoints de auth para evitar fuerza bruta.
