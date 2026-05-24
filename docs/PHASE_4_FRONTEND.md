# Fase 4 — Frontend base con Leptos conectado al backend Axum

## 1. Objetivo de la fase
Implementar el frontend base del MVP con Leptos, conectado al backend Axum, permitiendo visualizar productos, consultar categorías, crear productos y ver el detalle de un producto.

## 2. Stack utilizado
- Frontend: Rust + Leptos (Client-Side Rendering)
- Construcción y empaquetado: Trunk
- Peticiones HTTP: reqwest
- Estilos: CSS vanilla básico

## 3. Estructura frontend creada
Se ha establecido la arquitectura base del proyecto Leptos, dividida en `api`, `components`, `models`, y `pages`.

## 4. Conexión con backend Axum
El frontend se comunica correctamente con el backend expuesto en `http://127.0.0.1:3000` mediante el uso de `reqwest` en `src/api`.

## 5. Pantallas implementadas
- **Home:** Listado general de productos.
- **Detalle de Producto:** Vista de un producto específico.
- **Publicar Producto:** Formulario interactivo.

## 6. Componentes creados
- `ProductCard`
- `ErrorMessage`
- `Loading`

## 7. Llamadas API realizadas
- `GET /health` (Indirectamente para pruebas)
- `GET /categories`
- `GET /products`
- `GET /products/:id`
- `POST /products`

## 8. Validaciones aplicadas
El formulario de creación de productos previene envíos con campos requeridos vacíos (título, descripción, ubicación) y precios negativos, obligando además a seleccionar una categoría válida.

## 9. Manejo de errores
- Mensajes visuales (`<ErrorMessage/>`) al fallar una llamada API.
- `<ErrorBoundary>` y `<Suspense>` nativos de Leptos para manejar estados de carga.
- Mensajes de validación en los formularios.

## 10. Pruebas recomendadas
Ejecutar `cargo run` en `backend/` seguido de `trunk serve` en `frontend/` y verificar los flujos principales (listar, ver, crear). 

## 11. Riesgos detectados
- En esta fase se envía un `user_id = 1` de forma dura porque la autenticación todavía no se ha implementado. Esto está aceptado temporalmente.
- Si Trunk falla, verificar que `wasm-unknown-unknown` target está instalado.

## 12. Próxima fase recomendada
Fase 5 — Implementación del sistema de Autenticación (Login/Registro) con JWT y gestión real de usuarios en frontend y backend.
