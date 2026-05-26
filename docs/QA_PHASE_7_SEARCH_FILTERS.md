# QA: Fase 7 - Búsqueda y Filtros de Productos

## Pruebas Backend (Axum + SQLite)
1. **Endpoint `GET /products` sin filtros**
   - Acción: `Invoke-RestMethod -Uri "http://127.0.0.1:3000/products" -Method GET`
   - Resultado esperado: Devuelve array JSON con todos los productos ordenados por `created_at DESC`.
   - Estado: ✅ Pasa

2. **Búsqueda por texto (`search`)**
   - Acción: `Invoke-RestMethod -Uri "http://127.0.0.1:3000/products?search=iphone" -Method GET`
   - Resultado esperado: Devuelve productos cuyo título o descripción contiene "iphone" (insensible a mayúsculas gracias a SQLite LIKE).
   - Estado: ✅ Pasa

3. **Filtrado combinado**
   - Acción: `Invoke-RestMethod -Uri "http://127.0.0.1:3000/products?category_id=1&min_price=100&max_price=500" -Method GET`
   - Resultado esperado: Devuelve productos de categoría 1, con precio entre 100 y 500.
   - Estado: ✅ Pasa

4. **Validación SQL**
   - Acción: Observación del código en `product_repository.rs`.
   - Resultado esperado: No hay concatenación directa de parámetros del usuario; se usa `query_builder.push_bind`.
   - Estado: ✅ Pasa

## Pruebas Frontend (Leptos)
1. **Renderizado de Componentes**
   - Acción: Cargar `http://127.0.0.1:8081/` en el navegador.
   - Resultado esperado: La barra de búsqueda y el panel de filtros se muestran correctamente.
   - Estado: ✅ Pasa

2. **Búsqueda interactiva**
   - Acción: Escribir "silla" en la barra y presionar Enter o hacer clic en "Buscar".
   - Resultado esperado: La lista se actualiza solo con productos que contengan "silla".
   - Estado: ✅ Pasa

3. **Aplicación de Filtros**
   - Acción: Rellenar "Precio Mín: 10", "Precio Máx: 50" y pulsar "Aplicar Filtros".
   - Resultado esperado: Petición `GET /products?min_price=10&max_price=50`. Lista se actualiza.
   - Estado: ✅ Pasa

4. **Limpieza de Filtros**
   - Acción: Pulsar "Limpiar".
   - Resultado esperado: Los campos de input se vacían/resetean y se hace petición sin query params.
   - Estado: ✅ Pasa

5. **Empty State**
   - Acción: Buscar un término inexistente (ej. "xxxxxxyyyyzzzz").
   - Resultado esperado: Se muestra el mensaje "No hay productos disponibles por el momento con estos filtros."
   - Estado: ✅ Pasa
