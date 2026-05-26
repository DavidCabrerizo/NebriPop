# Fase 7 — Búsqueda y Filtros de Productos

## 1. Objetivo de la fase
Implementar búsqueda y filtros de productos en NebriPop, permitiendo buscar por texto y filtrar por categoría, precio, estado, ubicación y estado de publicación. Todo debe guardar relación directa con la base de datos SQLite y estar libre de ficheros estáticos temporales.

## 2. Plan de implementación aprobado
El plan de implementación fue propuesto por el Agent Manager y aprobado antes de ejecutar código. Cubría la modificación de repositorios y handlers de Axum, así como componentes de Leptos en el Frontend.

## 3. Filtros implementados
- Búsqueda de texto (`title` o `description`).
- Categoría.
- Precio Mínimo y Máximo.
- Condición del producto.
- Ubicación.
- Estado de publicación.
- Ordenación por fecha o precio.

## 4. Query params utilizados
- `search`
- `category_id`
- `min_price`
- `max_price`
- `condition`
- `location`
- `status`
- `sort`

## 5. Cambios en backend
- Introducción de `ProductFiltersDto` y extracción a través de `axum::extract::Query`.
- Refactorización de `ProductRepository::find_all` para adoptar un enfoque de construcción dinámica de consultas utilizando `sqlx::QueryBuilder`.

## 6. Cambios en frontend
- Creación de componentes `SearchBar` y `ProductFilter`.
- Refactorización de `pages/home.rs` para centralizar la gestión de estado de los filtros mediante signals y triggers.
- Codificación en URL de los query params en `products_api.rs`.

## 7. Cambios en SQLite
- No se han realizado cambios en `schema.sql` en esta iteración ya que los índices existentes (`idx_products_category_id`, `idx_products_price`, `idx_products_status`, etc.) son adecuados y suficientes para las necesidades de filtrado de este MVP.

## 8. Validaciones aplicadas
- Validación estricta en Rust de las cláusulas `ORDER BY` mediante match block.
- Evitación de inyección de valores manuales mediante query bindings directos.
- Control de resultados vacíos en Frontend con renderizado de un *Empty State* adecuado.

## 9. Seguridad
- Ausencia total de concatenación insegura en sentencias SQL.
- Datos escapados naturalmente por Leptos y encodeados en URL para llamadas GET.
- Para más información, consultar el [Documento de Seguridad](SECURITY_PHASE_7_SEARCH_FILTERS.md).

## 10. Pruebas realizadas
- Verificación del compilado (`cargo check`) tanto en back como front.
- Pruebas conceptuales documentadas de filtros simples y combinados contra API local.
- Para más información, consultar el [Plan QA](QA_PHASE_7_SEARCH_FILTERS.md).

## 11. Resultado final
Fase de búsqueda y filtrado completada exitosamente. La funcionalidad central de exploración de productos en un marketplace se encuentra estable y operativa.

## 12. Próxima fase recomendada
**Fase 8**: Chat, Favoritos, o Sistema de Interacciones. Dependiendo de los requerimientos de producto.
