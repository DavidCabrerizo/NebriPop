# Informe de Pruebas (QA Testing Report) - MVP NebriPop

## 1. Pruebas de Backend
**Estado:** ✅ Aprobado

- **Compilación:** El backend compila correctamente sin errores críticos (`cargo check` y `cargo build`).
- **Arranque:** El servidor Axum levanta correctamente en `http://127.0.0.1:3000` con `cargo run`.
- **Endpoints Básicos:**
  - `GET /health`: Responde HTTP 200 OK.
  - `GET /categories`: Devuelve la lista de categorías (formato JSON válido).
  - `GET /products`: Devuelve el listado de productos, con soporte para paginación y filtros.
  - `GET /products/:id`: Retorna el detalle correcto de un producto.
  - `POST /products`: Creación de producto validada correctamente.
  - `GET /users/:id`: Retorna la información pública del usuario.
  - `GET /users/:id/products`: Retorna los productos de un usuario.
- **Funcionalidades Específicas:**
  - **Subida de Imágenes:** `POST /upload` permite cargar imágenes y asignarlas a productos, limitando tipos MIME (JPEG, PNG).
  - **Mensajería:** `GET /messages` y `POST /messages` funcionan correctamente, permitiendo el intercambio entre usuarios.
  - **Favoritos:** Añadir y eliminar favoritos opera sin errores (`POST /favorites`, `DELETE /favorites`).
- **Persistencia de Datos:**
  - Se confirmó el uso exclusivo de SQLite (`database/nebripop.db`).
  - No existen archivos `.json` actuando como sistema de persistencia (como `users.json`, `products.json`, etc.). Todo persiste tras reinicios.

## 2. Pruebas de Frontend
**Estado:** ✅ Aprobado

- **Compilación y Arranque:** Trunk empaqueta la aplicación Leptos y levanta el servidor de desarrollo en `http://127.0.0.1:8081` sin errores.
- **Navegación e Interfaz:**
  - **Home:** Carga correctamente el listado inicial de productos.
  - **Detalle de Producto:** La vista completa carga el título, precio, descripción, datos del vendedor e imágenes.
  - **Autenticación:** Las pantallas de Registro e Inicio de sesión operan y muestran alertas (éxito/error).
  - **Creación de Productos:** El formulario permite insertar un nuevo producto con campos obligatorios validados (precio mayor o igual a 0, título no vacío, estado).
  - **Búsqueda y Filtros:** El cuadro de búsqueda filtra por texto y categoría correctamente, refrescando la grilla de productos sin recargar la página.
  - **Mensajería y Favoritos:** Iconos y vistas específicas cargan la información desde el backend y la renderizan correctamente.
  - **Experiencia Visual (UX/UI):** No hay enlaces rotos. Los errores devuelven feedback visual. Las imágenes usan "placeholders" si fallan en cargar (no se ven imágenes rotas).

## 3. Pruebas de Base de Datos
**Estado:** ✅ Aprobado

- **Esquema (`schema.sql`):** 
  - Se pueden generar todas las tablas sin errores (users, categories, products, product_images, favorites, messages, blocks, deleted_conversations).
  - Contiene claves foráneas, índices apropiados (para `category_id`, `user_id`, etc.) y restricciones (como `price >= 0`).
- **Semillas (`seeds.sql`):**
  - Los datos de prueba se insertan correctamente cumpliendo todas las Foreign Keys.
  - No incluye datos personales reales (solo usuarios "John Doe" / pruebas).
- **Relaciones:** Las eliminaciones en cascada y restricciones (`ON DELETE CASCADE`, `ON DELETE SET NULL`) funcionan correctamente al eliminar usuarios o categorías.

## 4. Conclusión
El MVP se encuentra estable. Las integraciones Full-Stack operan como se espera, validando los requerimientos de la Fase 10. QAAgent emite su resultado:

**Resultado:** QA FINAL APROBADO ✅
