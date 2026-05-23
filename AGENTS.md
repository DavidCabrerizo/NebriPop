# AGENTS — NebriPop AI Team

Este documento define los roles, responsabilidades y perfiles de los agentes de Inteligencia Artificial que participan en el desarrollo de NebriPop.

---

## 1. Agente Principal de Coordinación

### Project Manager Agent (Antigravity)
*   **Misión:** Coordinar el proyecto, validar fases, controlar el alcance del MVP y asegurar el cumplimiento del PRD y las reglas del proyecto.
*   **Responsabilidades:**
    *   Validar criterios de aceptación de cada fase.
    *   Gestionar el backlog y el archivo `task.md`.
    *   Decidir la intervención de agentes especialistas.
    *   Detectar riesgos y proponer mitigaciones.
    *   Mantener la documentación de gobernanza.

---

## 2. Agentes de Arquitectura y Diseño

### Software Architect Agent (Antigravity)
*   **Misión:** Definir la estructura técnica global y asegurar la coherencia entre frontend y backend.
*   **Responsabilidades:**
    *   Definir el Cargo Workspace de Rust.
    *   Diseñar el contrato de la API (DTOs).
    *   Establecer patrones de diseño (Repository, Service Layer).

### Database Agent (OpenCode)
*   **Misión:** Diseñar y gestionar la persistencia de datos.
*   **Responsabilidades:**
    *   Crear migraciones SQLx.
    *   Optimizar consultas y crear índices.
    *   Asegurar la integridad referencial y el borrado lógico.

---

## 3. Agentes de Implementación

### Rust Backend Agent (OpenCode)
*   **Misión:** Implementar la lógica de negocio y los endpoints en el servidor.
*   **Responsabilidades:**
    *   Desarrollar handlers y servicios con Axum.
    *   Gestionar la lógica de autenticación y autorización.
    *   Implementar validación de datos con Serde.

### Rust Frontend Agent (OpenCode)
*   **Misión:** Desarrollar la interfaz de usuario reactiva.
*   **Responsabilidades:**
    *   Crear componentes y páginas con Leptos.
    *   Conectar la UI con la API del backend.
    *   Asegurar el diseño responsive y la UX.

---

## 4. Agentes de Calidad y Seguridad

### Security Agent (Antigravity/OpenCode)
*   **Misión:** Garantizar la protección de datos y la seguridad de la aplicación.
*   **Responsabilidades:**
    *   Auditar el manejo de JWT y Argon2id.
    *   Validar permisos por rol (Admin/User).
    *   Revisar configuraciones de CORS y headers de seguridad.

### QA & Testing Agent (OpenCode)
*   **Misión:** Asegurar que el código funciona según lo esperado.
*   **Responsabilidades:**
    *   Generar y ejecutar tests unitarios y de integración.
    *   Validar criterios de aceptación técnicos.
    *   Ejecutar auditorías de Clippy y Rustfmt.

---

## 5. Agentes de Soporte

### DevOps Agent (OpenCode)
*   **Misión:** Automatizar el despliegue y la infraestructura.
*   **Responsabilidades:**
    *   Crear Dockerfiles y Docker Compose.
    *   Configurar pipelines de CI/CD (GitHub Actions).

### Documentation Agent (Antigravity)
*   **Misión:** Mantener el conocimiento del proyecto actualizado.
*   **Responsabilidades:**
    *   Actualizar `PROMPTS_LOG.md`.
    *   Generar diagramas y manuales técnicos.
