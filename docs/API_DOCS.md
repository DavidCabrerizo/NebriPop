# Documentación API - NebriPop (Actualizado Fase 5B)

## 1. Productos

### 1.1 Obtener todos los productos
- **Método:** `GET`
- **Ruta:** `/products`
- **Respuesta:**
  ```json
  [
    {
      "id": 1,
      "user_id": 1,
      "category_id": 2,
      "title": "Bicicleta",
      "description": "Casi nueva",
      "price": 100.0,
      "condition": "like_new",
      "location": "Madrid",
      "status": "available",
      "main_image_url": "uploads/products/1/imagen_1.jpg",
      "created_at": "2026-05-24T12:00:00Z",
      "updated_at": "2026-05-24T12:00:00Z",
      "images": null
    }
  ]
  ```

### 1.2 Obtener producto por ID
- **Método:** `GET`
- **Ruta:** `/products/:id`
- **Respuesta:**
  ```json
  {
    "id": 1,
    "user_id": 1,
    "category_id": 2,
    "title": "Bicicleta",
    "description": "Casi nueva",
    "price": 100.0,
    "condition": "like_new",
    "location": "Madrid",
    "status": "available",
    "main_image_url": "uploads/products/1/imagen_1.jpg",
    "created_at": "2026-05-24T12:00:00Z",
    "updated_at": "2026-05-24T12:00:00Z",
    "images": [
      {
        "id": 1,
        "product_id": 1,
        "image_url": "uploads/products/1/imagen_1.jpg",
        "position": 0,
        "created_at": "2026-05-24T12:00:00Z"
      }
    ]
  }
  ```

### 1.3 Crear producto
- **Método:** `POST`
- **Ruta:** `/products`
- **Body (JSON):**
  ```json
  {
    "user_id": 1,
    "category_id": 2,
    "title": "Bicicleta",
    "description": "Casi nueva",
    "price": 100.0,
    "condition": "like_new",
    "location": "Madrid"
  }
  ```

### 1.4 Añadir imágenes al producto
- **Método:** `POST`
- **Ruta:** `/products/:id/images`
- **Content-Type:** `multipart/form-data`
- **Campos:**
  - `file`: (múltiple) Archivos locales en formato jpg, jpeg o png.
  - `image_url`: (texto) URL externa opcional.
- **Respuesta:**
  ```json
  {
    "message": "Imágenes añadidas correctamente",
    "images": [
      {
        "id": 1,
        "product_id": 1,
        "image_url": "uploads/products/1/imagen_1.jpg",
        "position": 0,
        "created_at": "2026-05-24T12:05:00Z"
      }
    ]
  }
  ```

## 2. Archivos Estáticos
- **Ruta:** `/uploads/*`
- **Descripción:** Sirve públicamente los archivos subidos localmente a través de la API. Ejemplo: `http://127.0.0.1:3000/uploads/products/1/imagen_1.jpg`.
