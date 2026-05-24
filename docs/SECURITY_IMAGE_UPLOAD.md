# Análisis de Seguridad Subida de Imágenes - Fase 5B

## Controles Implementados
- **Validación de extensión:** Se valida que la extensión del archivo sea únicamente `jpg`, `jpeg` o `png`. (Nota: La validación es a nivel de extensión del archivo enviado).
- **Tamaño máximo:** El tamaño máximo por archivo se ha fijado en 5 MB para evitar ataques de denegación de servicio (DoS) por llenado de disco.
- **Sanitización del nombre de archivo:** El backend ignora el nombre del archivo enviado por el usuario y genera un nombre seguro en base a un contador (`imagen_1.jpg`, `imagen_2.png`, etc.). Esto mitiga posibles ataques de *Path Traversal* y previene el uso de caracteres especiales peligrosos o extensiones disfrazadas.
- **Evitar Path Traversal:** Al forzar la nomenclatura del archivo y construir la ruta de guardado localmente de forma controlada (`uploads/products/{id}/imagen_X.ext`), se impide completamente el *Path Traversal*.
- **Archivos ejecutables:** Se rechazan archivos que no terminen en las extensiones permitidas, evitando subir `.exe`, `.sh`, `.php`, `.js`, `.html`, etc.
- **Persistencia Segura:** Las rutas se guardan de forma paramétrica usando bind de SQLite y variables seguras en Rust, evitando SQL Injection. Las imágenes no se guardan como BLOB para evitar sobrecargar la base de datos.
- **Aislamiento del directorio:** Los archivos se guardan en un directorio `backend/uploads/products/`. El servidor estático de Axum restringe la entrega de contenido estrictamente a esta carpeta y la sirve con los MIME types apropiados.

## Recomendaciones para Producción
1. **Validación de MIME-type:** Implementar validación real del MIME-type (magic numbers/cabeceras del archivo) leyendo los primeros bytes del binario, en lugar de confiar únicamente en la extensión.
2. **CDN o S3:** Al escalar a producción, no almacenar archivos localmente. Utilizar un servicio de almacenamiento de objetos externo (S3) para evitar pérdida de datos si se reinicia el contenedor del backend, y para aislar posibles vulnerabilidades.
3. **Limitar subida por URL externa:** Las URLs externas provistas por el usuario pueden ser usadas para SSRF. Debería limitarse su uso o procesar la imagen de manera asíncrona si el servidor las descarga. En esta fase la URL se guarda y se confía al cliente para renderizarla.
4. **Git Ignore:** Las imágenes subidas por los usuarios durante las pruebas locales (`backend/uploads/*`) han sido excluidas mediante `.gitignore` para no ser versionadas en el repositorio.
