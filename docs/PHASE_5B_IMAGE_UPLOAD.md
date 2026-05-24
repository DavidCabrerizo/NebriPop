# Fase 5B — Gestión de imágenes de productos

## 1. Objetivo
Añadir soporte completo para la carga, persistencia y visualización de imágenes de productos en NebriPop (archivos locales y URLs externas).

## 2. Formatos permitidos
Solo se permiten archivos con extensiones `.jpg`, `.jpeg` y `.png`. El tamaño máximo por archivo es 5 MB.

## 3. Flujo de subida local
El frontend captura las imágenes con un `<input type="file" multiple>` y las envía mediante `multipart/form-data` al backend usando `web_sys::FormData`. El backend guarda los archivos físicamente en la ruta `backend/uploads/products/{product_id}/` generando nombres secuenciales (`imagen_1.jpg`, etc.) y registra sus rutas en SQLite.

## 4. Flujo por URL externa
El usuario puede pegar un enlace en un campo de texto adicional del formulario. El enlace se envía en el mismo form-data y el backend lo registra directamente en SQLite sin descargarlo.

## 5. Carpeta de almacenamiento
`backend/uploads/products/`

## 6. Relación con SQLite
La base de datos actualiza el campo `main_image_url` en la tabla `products` y almacena múltiples imágenes con su orden en la tabla vinculada `product_images` usando claves foráneas.

## 7. Endpoints creados o modificados
- `POST /products/:id/images` (nuevo): Recibe el multipart form data.
- `GET /products` (modificado): Asegura devolver `main_image_url`.
- `GET /products/:id` (modificado): Recupera de base de datos y adjunta el arreglo completo de `images`.

## 8. Visualización en listado
El componente `ProductCard` del frontend en Leptos mapea el campo `main_image_url` (si existe y es local lo prefija con `http://127.0.0.1:3000/`) y lo muestra en la tarjeta, o un placeholder estándar.

## 9. Visualización en detalle
El componente `ProductDetail` expone la imagen principal grande y construye una galería de miniaturas interactiva.

## 10. Validaciones
- Validaciones en frontend para extensiones permitidas.
- Validaciones de tamaño y extensión en backend antes de escribir en disco.
- Validación de que la categoría asociada al producto existe.

## 11. Riesgos de seguridad
Mitigados los ataques de Path Traversal autogenerando el nombre del archivo en el servidor. Mitigado el llenado de disco validando el límite de 5 MB por archivo.

## 12. Pruebas realizadas
Tests de integración en el entorno de desarrollo comprobando que se suben archivos reales, que se parsea bien el request y que las miniaturas renderizan. Base de datos comprueba integridad.

## 13. Resultado final
La funcionalidad de gestión de imágenes se ha completado correctamente en toda la pila tecnológica (frontend Leptos, backend Axum, SQLite) cumpliendo todos los requisitos solicitados en el PRD de la Fase 5.

## 14. Próximo paso recomendado
Continuar con la implementación de autenticación de usuarios (Registro y Login con JWT) para evitar asignar `user_id: 1` por defecto a todos los productos creados.
