# Gestión de Imágenes de Productos

En la Fase 5B de NebriPop se ha implementado el soporte completo para subir y mostrar imágenes asociadas a los productos.

## Características

1. **Subida local:** Los usuarios pueden seleccionar múltiples archivos desde su dispositivo (`.jpg`, `.jpeg`, `.png`) y subirlos al servidor. Estos se guardan en la carpeta física `backend/uploads/products/{product_id}/`.
2. **Imágenes por URL externa:** Los usuarios pueden proporcionar una URL de imagen externa que se almacena en la base de datos sin descargarse físicamente, minimizando el consumo de espacio y simplificando el proceso.
3. **Persistencia:** Las rutas relativas de las imágenes locales y las URLs de las imágenes externas se guardan en la tabla `product_images` de la base de datos SQLite. La imagen principal del producto se almacena en la tabla `products` en el campo `main_image_url`.
4. **Visualización:**
   - En el listado general (`ProductCard`) se muestra únicamente la imagen principal. Si el producto no tiene imagen, se muestra un placeholder por defecto.
   - En el detalle del producto (`ProductDetail`) se muestra la imagen principal destacada y un carrusel o listado de miniaturas del resto de imágenes.
5. **Servidor estático:** El backend expone la carpeta local `uploads/` mediante el enrutador de Axum y `tower_http::services::ServeDir`, permitiendo a los clientes acceder a las imágenes mediante la ruta `http://127.0.0.1:3000/uploads/products/...`.
