# QA Image Upload - Fase 5B

## Backend
- [x] POST `/products/:id/images` acepta `.jpg`, `.jpeg`, `.png`.
- [x] POST `/products/:id/images` rechaza otros formatos.
- [x] Rechaza archivo demasiado grande (límite establecido en 5 MB).
- [x] Guarda archivo físicamente en `backend/uploads/products/{product_id}/`.
- [x] Guarda ruta relativa en la base de datos SQLite (tabla `product_images`).
- [x] Actualiza `main_image_url` en la tabla `products` si es la primera imagen.
- [x] El backend sirve imágenes estáticas desde `/uploads/`.
- [x] `GET /products` incluye `main_image_url`.
- [x] `GET /products/:id` incluye la lista de imágenes `images: []`.

## Frontend
- [x] El formulario permite seleccionar una o varias imágenes locales.
- [x] El formulario permite añadir una URL externa.
- [x] El producto se crea correctamente y luego envía las imágenes.
- [x] Las imágenes se muestran correctamente en el listado general (`ProductCard`).
- [x] La imagen principal y las miniaturas se muestran en el detalle (`ProductDetail`).
- [x] Se muestra un placeholder si no hay imagen en el producto.
- [x] Las imágenes locales se muestran construyendo la URL absoluta con `http://127.0.0.1:3000/`.
- [x] No aparece el icono de imagen rota.

## Base de datos
- [x] `product_images` contiene las rutas (e.g., `uploads/products/1/imagen_1.jpg`).
- [x] `products.main_image_url` se actualiza correctamente al subir la primera imagen.
- [x] Ningún archivo JSON se utiliza como persistencia. Todo se almacena en SQLite.
