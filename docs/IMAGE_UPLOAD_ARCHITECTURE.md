# Arquitectura de Subida de Imágenes - Fase 5B

## 1. Flujo de Subida Local
- El frontend usa un `<input type="file" multiple>` para seleccionar imágenes.
- Se envían las imágenes mediante `multipart/form-data` a un endpoint específico: `POST /products/:id/images`.
- El backend procesa las partes, valida las extensiones (`.jpg`, `.jpeg`, `.png`) y guarda físicamente cada archivo en `backend/uploads/products/{product_id}/`.
- Se registra la ruta relativa (ej. `uploads/products/{id}/filename.jpg`) en la base de datos (tabla `product_images`).
- Si el producto no tiene `main_image_url`, se actualiza con la primera imagen subida.

## 2. Flujo de URL Externa
- El frontend permite ingresar una URL de imagen a través de `<input type="url">`.
- Se envía la URL como un campo dentro del mismo `multipart/form-data` o a través del mismo endpoint.
- El backend no descarga la imagen, sino que simplemente inserta el registro en `product_images` y, si procede, actualiza `main_image_url`.

## 3. Almacenamiento y Archivos Estáticos
- Los archivos subidos se guardarán en la carpeta local `backend/uploads/products/`.
- El framework Axum servirá estos archivos estáticos en la ruta `/uploads/` usando `ServeDir` (requiere `tower_http::services::ServeDir`).

## 4. Base de Datos
- Las rutas se guardan en la tabla `product_images` como texto.
- Se vinculan al producto a través del `product_id`.
- Se aprovecharán las tablas y esquemas existentes ya preparados.

## 5. Endpoints
- `POST /products` (existente, para crear los datos básicos del producto)
- `POST /products/:id/images` (nuevo, para añadir imágenes al producto ya creado)
- `GET /products` (existente, se actualiza para asegurar que devuelva `main_image_url`)
- `GET /products/:id` (existente, se actualiza para devolver un objeto de respuesta con `images`)
