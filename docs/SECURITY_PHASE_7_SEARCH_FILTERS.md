# Seguridad: Fase 7 - Búsqueda y Filtros de Productos

## Análisis de Riesgos y Mitigaciones Implementadas

### 1. Inyección SQL (SQL Injection)
- **Riesgo**: Un atacante podría insertar comandos SQL maliciosos a través de la barra de búsqueda o los filtros (ej: `search=iphone'; DROP TABLE products;--`).
- **Mitigación**: Se ha implementado el uso exclusivo de `QueryBuilder::push_bind` en `ProductRepository::find_all`. Esto asegura que SQLx trata todos los inputs del usuario como datos (parámetros) y nunca como instrucciones SQL ejecutables.
- **Estado**: ✅ Seguro.

### 2. Manipulación del Order By
- **Riesgo**: El parámetro `sort` podría usarse para manipular la estructura de la consulta si se concatena directamente.
- **Mitigación**: El valor de `sort` se valida mediante un bloque `match` que mapea valores predefinidos (`"price_asc"`, `"price_desc"`) a cláusulas SQL estáticas hardcodeadas (`ORDER BY price ASC`, `ORDER BY price DESC`). Cualquier valor no reconocido cae en el caso por defecto (`ORDER BY created_at DESC`).
- **Estado**: ✅ Seguro.

### 3. Exposición de Datos (Data Exposure)
- **Riesgo**: Que la búsqueda devuelva información privada de usuarios o productos ocultos.
- **Mitigación**: La consulta base es `SELECT id, user_id, category_id, title, description, price, condition, location, status, main_image_url, created_at, updated_at FROM products`. Solo devuelve datos públicos del producto. No se hace JOIN automático exponiendo hashes de contraseñas.
- **Estado**: ✅ Seguro.

### 4. Denegación de Servicio (DoS) por Búsquedas Complejas
- **Riesgo**: Búsquedas `LIKE %...%` pueden ser costosas en tablas muy grandes.
- **Mitigación**: Para un MVP con SQLite el rendimiento es aceptable. Se han creado los índices básicos en `schema.sql` (`idx_products_category_id`, `idx_products_status`, `idx_products_price`). Como futura mejora (cuando haya gran volumen de datos), se sugiere implementar SQLite FTS5 (Full Text Search) para evitar escaneos completos de tabla.
- **Estado**: ⚠️ Riesgo aceptado para la fase MVP.

### 5. Inyección XSS en Frontend
- **Riesgo**: Reflejar términos de búsqueda con scripts maliciosos en la UI.
- **Mitigación**: Leptos escapa automáticamente el HTML por defecto al insertar texto en las vistas mediante `{variables}` o `prop:value`. Además, `urlencoding::encode` se utiliza en las peticiones HTTP.
- **Estado**: ✅ Seguro.
