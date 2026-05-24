# API Documentation NebriPop

## Endpoints de la API (Fase 3)

### GET /health
**Descripción**: Comprueba que el servidor está en funcionamiento.
- **Respuesta esperada** (200 OK):
```json
{
  "status": "ok",
  "message": "NebriPop backend funcionando"
}
```

### GET /categories
**Descripción**: Devuelve la lista de categorías disponibles desde SQLite.
- **Respuesta esperada** (200 OK):
```json
[
  {
    "id": 1,
    "name": "Electrónica",
    "created_at": "2026-05-24T10:00:00"
  }
]
```

### GET /products
**Descripción**: Devuelve la lista de productos disponibles, ordenados por fecha de creación descendente.
- **Respuesta esperada** (200 OK):
```json
[
  {
    "id": 1,
    "user_id": 1,
    "category_id": 1,
    "title": "iPhone 12 usado",
    "description": "iPhone 12 en buen estado",
    "price": 250.0,
    "condition": "used",
    "location": "Madrid",
    "status": "available",
    "main_image_url": null,
    "created_at": "2026-05-24T10:00:00",
    "updated_at": "2026-05-24T10:00:00"
  }
]
```

### GET /products/:id
**Descripción**: Devuelve el detalle de un producto específico.
- **Respuesta esperada** (200 OK): Mismo formato que un objeto de la lista de productos.
- **Errores posibles**:
  - `404 Not Found`: Si el producto no existe.
  ```json
  {"error": "Producto con ID 999 no encontrado"}
  ```

### POST /products
**Descripción**: Crea un nuevo producto.
- **Body esperado**:
```json
{
  "user_id": 1,
  "category_id": 1,
  "title": "iPhone 12 usado",
  "description": "iPhone 12 en buen estado",
  "price": 250.0,
  "condition": "used",
  "location": "Madrid",
  "main_image_url": null
}
```
- **Respuesta esperada** (200 OK): El producto creado (incluyendo `id` y `created_at`).
- **Errores posibles**:
  - `400 Bad Request`: Si fallan validaciones de entrada (ej. precio negativo, categoría no existe, condición inválida).
  ```json
  {"error": "Error de validación: El precio debe ser mayor o igual que 0"}
  ```
