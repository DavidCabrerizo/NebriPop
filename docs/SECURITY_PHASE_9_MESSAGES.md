# Análisis de Seguridad Fase 9: Mensajería Básica

## Revisiones de Seguridad Aplicadas
1. **Validación de Datos (Input Validation)**: Se controlan los tamaños máximos de los mensajes (1000 caracteres) y se imposibilita el envío de mensajes vacíos.
2. **Inyección SQL**: El `message_repository` utiliza la función `.bind()` de SQLx para todas las inserciones y lecturas, previniendo cualquier tipo de inyección SQL.
3. **Control de Acceso Básico**: 
   - No se permite enviarse un mensaje a sí mismo.
   - Solo el `receiver_id` puede marcar un mensaje como leído mediante la ruta `PATCH`.
4. **Protección de Datos Sensibles**: El handler de mensajes no realiza ningún JOIN para extraer el `password_hash` ni ningún otro campo del modelo de usuario, devolviendo puramente los identificadores (IDs).

## Limitaciones y Riesgos (Aceptados en el MVP)
- **Autenticación en Frontend**: Al igual que en las fases anteriores, la identidad (`user_id`) sigue dependiendo del `localStorage` provisto por el frontend, lo que significa que un atacante puede alterar su ID e interceptar mensajes o enviarlos en nombre de otros si conoce su ID. Esto es una vulnerabilidad documentada y será abordada en una fase futura con JWT.

**Resultado de Seguridad:** Aprobado bajo las restricciones del MVP actual.
