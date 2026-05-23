# Seguridad: Riesgos Pre-MVP SQLite

Este documento evalúa los riesgos y consideraciones de seguridad exclusivos de la prueba `PreMVPNebriPopSQLiteV1`.

## Análisis de Riesgos Implementados

1. **Inyección SQL**: 
   - **Mitigación**: Resuelta. Se usan consultas parametrizadas (mediante el macro `params!` de `rusqlite`). No hay concatenación de strings en SQL.
   
2. **Campos vacíos o malformados**: 
   - **Mitigación**: Se han implementado comprobaciones básicas con `.trim().is_empty()` en los campos de texto, y `price <= 0.0`.
   - **Riesgo residual**: Aún se pueden enviar strings gigantes. Para el MVP real será necesario limitar la longitud (ej: `description` máximo 500 caracteres).

3. **Autenticación y Autorización**:
   - **Estado Actual**: Inexistente. Cualquier persona con acceso al puerto 3000 puede crear productos a nombre de cualquier `contact`.
   - **Riesgo**: Suplantación de identidad.
   - **Mitigación futura**: En el MVP, los endpoints mutables requerirán un JWT o sesión de usuario validada (Middleware).

4. **XSS (Cross-Site Scripting)**:
   - **Estado Actual**: Leptos escapa automáticamente el texto insertado en nodos DOM normales, mitigando la mayoría del XSS en la lectura.
   
5. **CORS**:
   - **Estado Actual**: Abierto (`Any`) para la prueba. 
   - **Mitigación futura**: En producción, restringir el origen únicamente al dominio legítimo del frontend.
