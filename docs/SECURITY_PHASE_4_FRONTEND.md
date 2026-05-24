# Análisis de Seguridad - Fase 4 (Frontend)

## Riesgos Revisados
- **Exposición de Secretos:** Se ha verificado que no existan tokens, contraseñas o claves API hardcodeadas en el código de Leptos.
- **Configuración de CORS:** El backend en Fase 3 debe permitir peticiones desde `http://127.0.0.1:8081`. Esto es seguro para el entorno de desarrollo local.
- **Validación de Entradas:** Aunque el backend ya valida (Fase 3), el frontend añade una capa de validación para evitar enviar datos basura, mejorando la UX. No reemplaza la seguridad del backend.
- **Autenticación Falsa (Temporal):** Se envía un `user_id = 1` de manera fija desde el frontend al crear productos. Esto está documentado como riesgo aceptado del MVP temporal, ya que el sistema de Login será abordado en futuras fases. **No** se está implementando autenticación falsa ni guardando claves en localStorage.
- **XSS (Cross-Site Scripting):** Leptos escapa automáticamente el texto insertado en el DOM, protegiendo contra inyección básica de scripts en campos como "título" o "descripción" renderizados posteriormente.
- **Imágenes Externas:** El campo `main_image_url` es de texto libre. Si el usuario ingresa un enlace malicioso, Leptos lo tratará como string o URL de la etiqueta `<img>`.

## Veredicto Inicial
Seguro para entorno de desarrollo y MVP, a la espera de la implementación real de autenticación.
