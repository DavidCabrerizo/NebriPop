# Fase 8 — Sistema de favoritos

## 1. Objetivo de la fase
Implementar el sistema de favoritos en NebriPop permitiendo a los usuarios guardar y gestionar productos que les interesan de manera persistente en SQLite.

## 2. Plan de implementación aprobado
Se aprobó el plan elaborado por `RustArchitectAgent` documentado en el artifact interactivo `implementation_plan.md`, focalizándose en añadir la persistencia vía DTOs, Repositories, Handlers, y la conexión en el frontend con Leptos y reqwest.

## 3. Funcionamiento de favoritos
Un usuario autenticado (simulado por el `user_id` del localStorage) puede hacer clic en un icono de corazón sobre cualquier producto (listado o detalle). Al hacerlo, el backend inserta el favorito. El botón se vuelve rojo. Un segundo clic lo elimina. El usuario tiene un apartado "Mis Favoritos" en la barra de navegación para consultar la lista completa.

## 4. Endpoints implementados
- `GET /users/:id/favorites`: Devuelve `Vec<Product>` correspondientes al usuario.
- `POST /favorites`: Añade el favorito.
- `DELETE /favorites/:product_id?user_id=X`: Elimina el favorito.
- `GET /favorites/check?user_id=X&product_id=Y`: Comprueba el estado del favorito.

## 5. Cambios en backend
Se crearon `favorite_dto.rs`, `favorite_repository.rs`, `favorites_handler.rs`, y `favorites.rs` (rutas). Se añadieron a sus respectivos módulos `mod.rs` manteniendo la arquitectura por capas limpia.

## 6. Cambios en frontend
Se crearon `models/favorite.rs`, `api/favorites_api.rs`, `components/favorite_button.rs`, y `pages/favorites.rs`. Se integró el `FavoriteButton` en `ProductCard` y `ProductDetail`. Se añadió la ruta al `Router` y a la barra superior.

## 7. Cambios en SQLite
**Ninguno requerido.** Se verificó que la tabla `favorites` ya existía desde la Fase 2, con la estructura `id`, `user_id`, `product_id`, y la limitación `UNIQUE(user_id, product_id)`.

## 8. Validaciones aplicadas
El Frontend valida la existencia de la sesión antes de enviar peticiones (muestra `alert` si no está logueado). El Backend confía en la restricción Unique para prevenir duplicidad, retornando un error validado si se intenta forzar.

## 9. Seguridad
Las peticiones al backend están parametrizadas contra SQL Injection. No hay almacenamiento en JSON. Se ha documentado la carencia de JWT/Tokens como un punto a mejorar cuando el MVP pase a producción real. (Ver `SECURITY_PHASE_8_FAVORITES.md`)

## 10. Pruebas realizadas
Se verificaron los estados renderizados del frontend y la compilación exitosa (`cargo check` completado sin errores lógicos o léxicos en ambas partes). (Ver `QA_PHASE_8_FAVORITES.md`)

## 11. Resultado final
Fase completada. El CRUD de favoritos funciona y se alinea con la estética y funcionalidades del MVP tipo Wallapop.

## 12. Próxima fase recomendada
Se recomienda avanzar a la Fase 9: Refactorización de Auth a un sistema robusto, o bien iniciar la implementación del chat / mensajes privados (si se considera MVP final).
