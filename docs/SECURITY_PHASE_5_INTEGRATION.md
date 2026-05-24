# Análisis de Seguridad Fase 5: Integración Completa Frontend + Backend + SQLite

Como parte de la **Fase 5**, se ha realizado una revisión de seguridad del proyecto NebriPop centrándose en los siguientes puntos clave:

## 1. Exposición de Secretos
- **Backend y Frontend:** Se ha verificado que no existen contraseñas, claves API ni secretos expuestos en el código fuente.
- **Control de versiones:** El archivo `.env` está correctamente excluido mediante el `.gitignore`. Solo existe un archivo `.env.example` en el repositorio, el cual no contiene información sensible.

## 2. Prevención de Inyección SQL
- **Consultas Parametrizadas:** Las consultas a la base de datos dentro de `backend/src/repositories/` (como en `product_repository.rs` y `category_repository.rs`) emplean el método `.bind()` provisto por `sqlx`. Esto garantiza que los valores se envían como parámetros al motor SQLite, anulando así el riesgo de ataques por inyección SQL.

## 3. Configuración CORS
- **Situación Actual:** En `backend/src/main.rs`, la política de CORS se encuentra configurada actualmente con `.allow_origin(Any)`.
- **Evaluación de Riesgo:** Para los límites de este MVP y desarrollo local, es un ajuste funcional y seguro.
- **Recomendación a futuro:** Si el sistema pasa a producción, se debe limitar estrictamente al origen del frontend (`http://127.0.0.1:8081` o dominio real) para reducir superficie de ataque.

## 4. Riesgo de XSS en URLs de imágenes
- **Uso de `main_image_url`:** En el frontend (`product_card.rs` y `product_detail.rs`), la URL de la imagen principal se renderiza de forma segura inyectando su valor al atributo `src` de las etiquetas `<img>` a través de Leptos.
- **Evaluación:** Leptos asocia el valor de `src` directamente en el DOM Node en lugar de parsearlo como código HTML vivo, por lo que cualquier script malicioso no se ejecutaría. Es seguro contra XSS.

## 5. Seguridad General del MVP
- Dentro del alcance de un MVP (sin sistema de autenticación complejo aún), la arquitectura muestra solidez técnica y defensas aplicadas correctamente donde corresponde.
- Se mantiene una separación clara de responsabilidades en la API, protegiendo las capas más sensibles como los repositorios y la base de datos.

---
**Conclusión:** La integración de la Fase 5 es robusta y cumple satisfactoriamente con los criterios de seguridad limitados al marco del MVP actual.
