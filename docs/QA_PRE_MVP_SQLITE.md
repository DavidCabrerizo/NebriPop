# Estrategia de Pruebas: QA Pre-MVP SQLite

## Objetivo
Validar que la base de datos persistente (SQLite) funciona en conjunto con Axum y Leptos en un entorno aislado sin afectar al código principal de NebriPop.

## Casos de Prueba - Backend

### 1. Healthcheck
- **Acción**: `Invoke-RestMethod -Uri http://127.0.0.1:3000/health -Method GET`
- **Resultado Esperado**: JSON con `{"status":"ok", "message":"Pre-MVP NebriPop backend funcionando"}`.

### 2. Obtener productos (GET)
- **Acción**: `Invoke-RestMethod -Uri http://127.0.0.1:3000/products -Method GET`
- **Resultado Esperado**: Array JSON con al menos los 2 productos iniciales (Libro y Calculadora).

### 3. Crear producto (POST)
- **Acción**: 
  ```powershell
  Invoke-RestMethod -Uri http://127.0.0.1:3000/products `
    -Method POST `
    -ContentType "application/json" `
    -Body '{"title":"Libro de Programación en Rust","description":"Libro usado para aprender Rust desde cero","price":20.0,"category":"Libros","condition":"Usado","location":"Campus Nebrija","contact":"usuario_prueba"}'
  ```
- **Resultado Esperado**: Objeto JSON del producto creado con su `id` y `created_at`.

### 4. Rechazos por validación
- Enviar payload con `title: ""` -> **Esperado**: Error 400 "title no puede estar vacío".
- Enviar payload con `price: -5.0` -> **Esperado**: Error 400 "price debe ser mayor que 0".

## Casos de Prueba - Frontend
1. Abrir `http://127.0.0.1:8081`. La app carga con el formulario y el listado de productos recuperados de la DB.
2. Rellenar formulario y pulsar "Publicar producto".
3. Aparece mensaje en verde "Producto publicado con éxito!".
4. El nuevo producto se visualiza instantáneamente al principio de la lista.
5. Dejar un campo en blanco y pulsar -> Mensaje rojo "Todos los campos son obligatorios".
6. Apagar el backend y pulsar publicar -> Mensaje rojo "Error de conexión con el backend".

## Pruebas de Integración y Persistencia
- Reiniciar el backend (Ctrl+C y `cargo run` de nuevo).
- Recargar el frontend en el navegador.
- **Resultado**: Los productos creados previamente siguen mostrándose intactos (lectura exitosa desde SQLite).
