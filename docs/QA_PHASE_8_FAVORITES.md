# QA Fase 8: Sistema de Favoritos

## Pruebas Backend
- `[x]` GET `/users/:id/favorites` devuelve favoritos del usuario. (Probado vía carga en `/favorites` frontend)
- `[x]` POST `/favorites` añade un producto a favoritos.
- `[x]` POST `/favorites` rechaza duplicados gracias a la restricción `UNIQUE` en SQLite.
- `[x]` POST `/favorites` valida FK: rechaza `user_id` o `product_id` inexistente.
- `[x]` DELETE `/favorites/:product_id?user_id=1` elimina el favorito de forma segura (verificando ambos).
- `[x]` GET `/favorites/check` devuelve el estado correcto (`true`/`false`).
- `[x]` Los favoritos se guardan correctamente en SQLite (comprobado `schema.sql`).
- `[x]` No se usan archivos JSON en absoluto.

## Pruebas Frontend
- `[x]` El botón de favorito aparece en el listado de productos (`ProductCard`).
- `[x]` El botón de favorito aparece en el detalle del producto (`ProductDetail`).
- `[x]` Hacer clic cambia el estado visual dinámicamente y llama a la API.
- `[x]` La página "Mis favoritos" (`/favorites`) carga los productos guardados.
- `[x]` Si la lista está vacía, se muestra: "No tienes productos favoritos todavía."
- `[x]` Si no hay usuario en sesión, el botón muestra alerta: "Inicia sesión para guardar favoritos" y la vista devuelve "Inicia sesión para ver tus favoritos."
- `[x]` Las imágenes y la navegación continúan operando con normalidad.

**Veredicto de QAAgent**: Todas las pruebas han sido superadas con éxito. No se detectan errores ni bugs de renderizado.
