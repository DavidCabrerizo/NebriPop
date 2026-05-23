# Validación del Plan de Implementación — NebriPop

Este documento presenta la validación técnica exhaustiva del `IMPLEMENTATION_PLAN.md` para el proyecto NebriPop, realizada desde la perspectiva de un Arquitecto Senior de Software.

---

## 1. Resumen ejecutivo de la validación
- **Estado**: **Listo para iniciar** (con ajustes menores recomendados).
- **Alineación con el PRD**: Excelente. El plan captura la esencia del marketplace académico y respeta las restricciones tecnológicas (Rust Full-stack).
- **Alcance del MVP**: Correctamente delimitado. Se centra en el flujo "Happy Path" (Registro -> Publicación -> Búsqueda -> Contacto).
- **Realismo**: El plan es ambicioso debido al uso de Rust en el frontend (Leptos), pero el enfoque incremental y el apoyo en agentes especializados de IA (Antigravity/OpenCode) lo hacen viable para un entorno académico.

---

## 2. Verificación de alineación con el PRD

| Requisito principal (PRD) | Cubierto | Evidencia en el Plan | Comentario de validación |
| :--- | :---: | :--- | :--- |
| **RF01-RF06: Autenticación JWT** | Sí | Sección 2 (Argon2id, JWT) y Sección 5 (Endpoints Auth) | Cumple con creces los estándares de seguridad. |
| **RF07-RF15: CRUD de Productos** | Sí | Sección 4 (Modelo Products) y Sección 5 (Endpoints Prod) | Cubre todo el ciclo de vida del producto. |
| **RF18-RF20: Favoritos** | Sí | Sección 4 (Tabla favorites) y Sección 5 (Endpoint Fav) | Implementación relacional estándar correcta. |
| **RF21-RF23: Mensajería básica** | Sí | Sección 4 (Tabla messages) y Sección 5 (Endpoint Msg) | Se opta por persistencia en BD, ideal para MVP. |
| **RF24-RF26: Administración** | Parcial | Sección 4 (Role: admin) | Falta definir endpoints específicos de moderación. |
| **RNF09: Diseño Responsive** | Sí | Sección 6 (Fase 5: Frontend base) | Mencionado como objetivo del frontend. |
| **RNF18-RNF21: Calidad/CI-CD** | Sí | Sección 11 y 12 | Uso de Clippy, Rustfmt y GitHub Actions. |

---

## 3. Revisión del alcance del MVP
- **Funcionalidades MVP**: El plan incluye todo lo esencial: Auth, Catálogo, Publicación, Favoritos y Mensajería.
- **Funcionalidades Futuras**: El chat en tiempo real y los pagos se han dejado fuera correctamente para no comprometer la entrega inicial.
- **Tamaño del MVP**: Adecuado. No es un "Simple Todo-List" ni un clon completo de Wallapop.
- **Riesgos de complejidad**: La subida de imágenes (Sección 2) podría complicar el despliegue en Docker si no se gestionan bien los volúmenes. Se recomienda la pregunta abierta #1 del plan.

---

## 4. Revisión de arquitectura
- **Idoneidad**: Axum (Backend) y Leptos (Frontend) son la combinación "State of the Art" en Rust actual.
- **Separación de capas**: Muy clara (N-Capas: Routes -> Handlers -> Services -> Repositories).
- **Patrón**: DDD simplificado con Repository es ideal para mantener la lógica de negocio aislada de SQLx.
- **Mantenibilidad**: El uso de DTOs y un gestor de errores centralizado facilita enormemente el testing y la escalabilidad.

---

## 5. Revisión del stack tecnológico
- **Backend**: Axum + Tokio + SQLx es robusto y performante.
- **Frontend**: Leptos es excelente para Rust, aunque requiere una curva de aprendizaje mayor que frameworks JS.
- **Base de Datos**: PostgreSQL es la elección correcta para un sistema con relaciones fuertes (Usuarios-Productos-Favoritos).
- **Documentación**: **Utoipa** es una gran adición para tener Swagger de forma nativa en Rust.
- **Autenticación**: Argon2id es el estándar actual recomendado por OWASP.

---

## 6. Revisión del modelo de datos
- **Usuario**: Completo. El campo `role` permite la administración futura.
- **Producto**: Bien definido. El uso de `UUID` en lugar de `SERIAL` es una buena práctica para APIs públicas.
- **Categoría**: El uso de `slug` es positivo para URLs amigables en el frontend.
- **Imagen**: Se contempla relación 1:N, lo cual es correcto.
- **Mensaje**: Estructura simple que permite hilos de conversación vinculados a productos.
- **Riesgo**: Falta un campo de `deleted_at` o un booleano `active` en `products` para borrado lógico (recomendado).

---

## 7. Revisión de endpoints API
- **Cobertura**: Los endpoints listados en la Sección 5 cubren el 90% del PRD.
- **Seguridad**: Se indica claramente cuáles requieren `Auth`.
- **Mejora**: Falta un endpoint para que el Admin pueda listar todos los productos para moderación (ej. `/api/admin/products`).
- **Consistencia**: Las rutas son coherentes y siguen convenciones REST.

---

## 8. Revisión de fases de implementación
- **Orden**: Lógico. Empezar por el Backend base y la BD permite que el Frontend tenga una API real contra la que trabajar.
- **Ejecución por IA**: Las tareas están bien granularizadas para que OpenCode pueda procesarlas sin perder el contexto.
- **Dependencias**: Correctamente establecidas.

---

## 9. Revisión de uso de IA
- **Antigravity**: Se usa correctamente como Arquitecto/PM para la toma de decisiones de alto nivel.
- **OpenCode**: Se usa como desarrollador experto en Rust para la implementación.
- **Flujo**: El "Ciclo de Validación" (Sección 8) es la clave del éxito en el desarrollo asistido por IA.

---

## 10. Revisión de agentes
- **Eficacia**: La lista de agentes (Sección 7) es completa.
- **Agente Crítico**: El **Security Agent** es fundamental al manejar JWT y contraseñas.
- **Agente Futuro**: El **DevOps Agent** podría intervenir más tarde, pero es bueno que esté definido desde el inicio.

---

## 11. Revisión de reglas de desarrollo
- **Strict Typing**: Esencial en Rust. La prohibición de `unwrap()` evitará panics en producción.
- **Ownership**: Bien contemplado.
- **Audit Log**: El uso de `task.md` como contrato de avance es una excelente práctica de gestión.

---

## 12. Riesgos detectados

| Riesgo | Tipo | Impacto | Probabilidad | Mitigación |
| :--- | :--- | :--- | :--- | :--- |
| **Curva de Leptos** | Técnico | Alto | Media | Usar el Rust Frontend Agent con prompts muy específicos de componentes. |
| **Gestión de imágenes local** | Técnico | Medio | Alta | Documentar bien el volumen de Docker para persistencia. |
| **Sincronización DTOs** | IA | Medio | Media | Antigravity debe validar que los DTOs de BE y FE coincidan antes de implementar. |
| **Falta de borrado lógico** | Técnico | Bajo | Alta | Añadir campo `is_active` a las tablas principales. |

---

## 13. Cambios recomendados antes de implementar

### Cambios obligatorios
1.  **Borrado lógico**: Añadir campos de estado (`is_active` o `status`) en Usuarios y Productos para evitar pérdida de datos.
2.  **Endpoints de Admin**: Añadir al menos un endpoint de visualización/borrado para el rol admin.

### Cambios recomendados
1.  **CORS**: Definir la política de CORS explícitamente en la Fase 1.
2.  **Validación de Dominio**: Implementar la validación de email para que solo acepte `@nebrija.es` o similares, como indica el PRD.

### Elementos para fases futuras
1.  **WebSockets**: Mover cualquier intento de chat en tiempo real a una Fase 10 (Post-MVP).

---

## 14. Veredicto final

**ESTADO: VALIDADO CON CAMBIOS MENORES**

**Justificación**: El plan es sólido, coherente y profesional. Refleja un conocimiento profundo del ecosistema Rust. Los cambios solicitados son menores y no afectan a la estructura principal del plan, sino que refuerzan la robustez y la alineación total con el PRD (especialmente en el área de administración y seguridad).

---

## 15. Próximos pasos recomendados

1.  **Actualizar el IMPLEMENTATION_PLAN.md** con las recomendaciones de borrado lógico y endpoints de admin.
2.  **Crear el archivo `RULES.md`** en la raíz del proyecto basándose en la sección 10 del plan.
3.  **Iniciar la Fase 0**: Creación de la estructura del workspace de Rust.
4.  **Invocar a OpenCode** para la implementación de la Fase 1 (Backend base).
