# Fase 3 — Backend base con Axum conectado a SQLite

## 1. Objetivo de la fase
Implementar el backend base del MVP de NebriPop utilizando Axum, conectado a la base de datos SQLite diseñada en la Fase 2, exponiendo los primeros endpoints necesarios para productos y categorías.

## 2. Stack utilizado
- Rust (Cargo)
- Axum (Framework Web)
- SQLx (Conexión asíncrona a SQLite)
- Tokio (Runtime asíncrono)
- Serde (Serialización/Deserialización JSON)

## 3. Estructura backend creada
Se ha utilizado un enfoque modular separando responsabilidades:
- `main.rs`: Inicialización de base de datos, CORS y arranque de servidor.
- `routes/`: Registro de endpoints.
- `handlers/`: Lógica de cada petición.
- `repositories/`: Acceso a base de datos usando SQLx.
- `models/`: Estructuras mapeadas desde tablas SQLite.
- `dto/`: Objetos de transferencia (payloads).
- `errors.rs`: Manejo unificado de errores y conversión a HTTP response.

## 4. Conexión con SQLite
El backend inicializa y lee la base de datos local `database/nebripop.db`. Si la tabla de usuarios no existe, inicializa automáticamente el schema (`schema.sql`) y los seeds (`seeds.sql`).

## 5. Endpoints implementados
- `GET /health`: Verificación del sistema.
- `GET /categories`: Lista de categorías.
- `GET /products`: Lista de productos.
- `GET /products/:id`: Detalle de producto por id.
- `POST /products`: Creación de producto.

## 6. Modelos y DTOs
- **Modelos**: `Category`, `Product`.
- **DTOs**: `CreateProductDto`.

## 7. Validaciones aplicadas
- Categoría existente, título y descripción obligatorios.
- Precio >= 0, condición restringida a valores válidos.
- Localización obligatoria.
- Errores de validación responden con `400 Bad Request`.

## 8. Manejo de errores
- Uso de enum `AppError` con `thiserror`.
- Respuestas estructuradas JSON.
- Ocultación de errores internos de base de datos en respuestas a cliente (500).

## 9. Pruebas recomendadas
Ver [QA_PHASE_3_BACKEND.md](./QA_PHASE_3_BACKEND.md).

## 10. Riesgos detectados
Ver [SECURITY_PHASE_3_BACKEND.md](./SECURITY_PHASE_3_BACKEND.md).

## 11. Próxima fase recomendada
Fase 4: Inicialización del Frontend Leptos y conexión con este API.
