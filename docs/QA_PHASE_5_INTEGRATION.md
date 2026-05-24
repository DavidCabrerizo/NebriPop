# Test Plan: Fase 5 - Integración Frontend + Backend + SQLite de NebriPop

## Executive Summary
Validar la integración completa de la aplicación NebriPop. Esta fase asegura que el frontend en Rust (Leptos/Yew u otro frame) se comunique correctamente con el backend en Rust (Axum), y que este, a su vez, persista y recupere la información desde la base de datos SQLite local.

## Test Scope
**In Scope:**
- Disponibilidad del backend (`GET /health`).
- Operaciones de lectura de categorías y productos (conexión Backend - SQLite).
- Creación de productos desde el frontend, procesamiento en backend y guardado en SQLite.
- Persistencia de datos (validación tras reinicio del servidor).
- Manejo de errores de red o backend caído en el frontend.
- Comprobación de existencia y correcta inicialización de `nebripop.db`.

**Out of Scope:**
- Pruebas de carga o estrés extremo.
- Autenticación o autorización (fuera del MVP actual).

## Environment
- **Backend:** Axum escuchando en `http://127.0.0.1:3000`.
- **Frontend:** WASM servido localmente.
- **Database:** SQLite archivo local en `./database/nebripop.db`.

## Checklist de Validaciones Teóricas (Code Review)

- [x] **Backend GET /health:** Configurado en `src/routes/mod.rs` apuntando al manejador `health_check`.
- [x] **Backend a DB:** Se usa `sqlx::SqlitePool` para las queries en los repositorios de `products` y `categories`.
- [x] **Frontend a Backend:** `src/api/client.rs` define el `API_BASE_URL` correctamente apuntando a `http://127.0.0.1:3000` y usa `reqwest` en llamadas asíncronas para `fetch_products`, `fetch_product_by_id`, y `create_product`.
- [x] **Manejo de Errores:** En `src/api/client.rs` existe `handle_network_error` para gestionar casos donde el backend está caído, manejando un "Error de conexión al servidor".
- [x] **Base de Datos:** El archivo físico `nebripop.db` existe en `/database/nebripop.db` y las configuraciones de la aplicación se enlazan con él correctamente.

---

## Test Cases (Ejecución Manual / QA)

### TC-INT-001: Verificación de Salud del Backend
**Priority:** P0 (Critical) | **Type:** Integration
**Steps:**
1. Levantar el backend.
2. Realizar petición `GET http://127.0.0.1:3000/health`.
**Expected:** Retorna status `200 OK`.

### TC-INT-002: Carga de Datos desde SQLite
**Priority:** P0 (Critical) | **Type:** Integration
**Steps:**
1. Abrir el frontend en el navegador.
2. Navegar a la página de inicio o lista de productos.
**Expected:** 
- La aplicación hace una llamada exitosa a `GET /products` y `GET /categories`.
- Se muestran los productos iniciales cargados en la interfaz desde la base de datos `nebripop.db`.

### TC-INT-003: Creación y Persistencia de Producto
**Priority:** P0 (Critical) | **Type:** Functional / Integration
**Steps:**
1. En el frontend, ir al formulario de creación de producto.
2. Rellenar los campos requeridos (Título, Descripción, Precio, Categoría) y enviar (POST).
3. Recargar la página principal y verificar si aparece el producto.
4. Detener y reiniciar el proceso del backend y volver a cargar el frontend.
**Expected:**
- Tras crear, la lista se actualiza mostrando el nuevo producto.
- Al reiniciar el backend, el producto creado sigue apareciendo (verificando persistencia en `nebripop.db`).

### TC-INT-004: Manejo de Errores - Backend Caído
**Priority:** P1 (High) | **Type:** Resilience
**Steps:**
1. Detener el proceso del backend.
2. En el frontend, intentar cargar la página principal o enviar un nuevo producto.
**Expected:**
- El frontend no debe colgarse ni mostrar un error de sistema.
- Se debe mostrar un mensaje amigable al usuario indicando el error de red o de conexión.

### TC-INT-005: Verificación de Base de Datos Local
**Priority:** P0 (Critical) | **Type:** System
**Steps:**
1. Ir al directorio `/database`.
2. Verificar la existencia y tamaño del archivo `nebripop.db`.
**Expected:** El archivo existe y no se resetea ni se borra automáticamente en cada arranque del servidor backend.

---

## Risk Assessment
| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Frontend no alcanza backend | M | H | `API_BASE_URL` en `client.rs` ya está verificado a `127.0.0.1:3000`. Asegurarse de arrancar ambos en los puertos correctos. |
| Errores de CORS | H | H | El backend en Rust (Axum) debe tener configurada una capa (middleware) de CORS para aceptar peticiones del puerto del frontend. |
| DB lock por accesos | L | M | Delegado a `sqlx::SqlitePool`, que maneja peticiones concurrentes a un archivo local de forma robusta. |

## Conclusión de la Auditoría Teórica
La arquitectura de integración está lista. Las rutas Axum y las llamadas API usando Reqwest en el Frontend están implementadas y conectan. A nivel de revisión de código, todos los requisitos de integración de la Fase 5 parecen cumplirse satisfactoriamente, listos para su validación manual por parte del tester o manager.
