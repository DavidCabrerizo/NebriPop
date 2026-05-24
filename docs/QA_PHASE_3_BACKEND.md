# QA Fase 3 Backend

## Checklist de Validación

- [x] El backend compila correctamente (`cargo check` / `cargo build`).
- [x] El servidor arranca correctamente con `cargo run` en el puerto 3000.
- [x] Endpoint `GET /health` responde correctamente 200 OK.
- [x] Endpoint `GET /categories` devuelve categorías desde SQLite.
- [x] Endpoint `GET /products` devuelve la lista de productos desde SQLite.
- [x] Endpoint `GET /products/:id` devuelve un producto existente.
- [x] Endpoint `GET /products/:id` devuelve error JSON con status 404 si no existe.
- [x] Endpoint `POST /products` crea un producto válido en SQLite.
- [x] Endpoint `POST /products` rechaza título vacío (status 400).
- [x] Endpoint `POST /products` rechaza descripción vacía (status 400).
- [x] Endpoint `POST /products` rechaza precio negativo (status 400).
- [x] Endpoint `POST /products` rechaza condition inválido (status 400).
- [x] Endpoint `POST /products` rechaza category_id inexistente (status 400).
- [x] Los errores se devuelven en formato JSON estructurado.
- [x] No se han implementado funcionalidades fuera de Fase 3 (ej. frontend, login).
- [x] No se ha modificado la carpeta `frontend/`.

## Comandos de Prueba (PowerShell)

### GET /health
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/health -Method GET
```

### GET /categories
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/categories -Method GET
```

### GET /products
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/products -Method GET
```

### GET /products/1
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/products/1 -Method GET
```

### POST /products
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/products `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"user_id":1,"category_id":1,"title":"iPhone 12 usado","description":"iPhone 12 en buen estado","price":250.0,"condition":"used","location":"Madrid","main_image_url":null}'
```
