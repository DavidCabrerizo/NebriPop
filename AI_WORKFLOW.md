# AI WORKFLOW — NebriPop Collaboration Protocol

Este documento define cómo colaboran Antigravity y OpenCode para asegurar un desarrollo estructurado y sin errores.

---

## 1. Ciclo de Trabajo Estándar

1.  **Planificación (Antigravity):**
    *   Revisa el `PRD.md` y el `IMPLEMENTATION_PLAN.md`.
    *   Define la tarea específica en el `task.md`.
    *   Genera un diseño técnico detallado (contrato API, estructura de archivos, lógica).
2.  **Implementación (OpenCode):**
    *   Recibe el diseño técnico de Antigravity.
    *   Genera el código Rust (backend o frontend).
    *   Implementa los tests correspondientes.
3.  **Refactorización y Validación (Antigravity + OpenCode):**
    *   Antigravity revisa el código generado buscando violaciones de `RULES.md` o principios SOLID.
    *   OpenCode aplica las correcciones solicitadas.
    *   Se ejecutan tests automatizados (`cargo test`).
4.  **Cierre de Tarea:**
    *   Antigravity marca la tarea como completada en `task.md`.
    *   Se documenta el cambio relevante en `PROMPTS_LOG.md`.

---

## 2. Matriz de Decisión de Herramientas

| Tarea | Herramienta | Acción |
| :--- | :--- | :--- |
| **Diseño de Arquitectura** | Antigravity | Crear diagramas y planes. |
| **Gestión de Errores** | Antigravity | Definir el Enum de errores y estrategia. |
| **Generación de Código** | OpenCode | Escribir lógica de handlers/servicios. |
| **Corrección de Errores** | OpenCode | Resolver fallos de compilación. |
| **Testing** | OpenCode | Crear unit/integration tests. |
| **Validación MVP** | Antigravity | Verificar cumplimiento de requisitos. |

---

## 3. Protocolo de Comunicación

*   **Contexto:** Siempre que se invoque a un agente para una tarea técnica, se debe proporcionar acceso al `PRD.md`, `IMPLEMENTATION_PLAN.md` y `RULES.md`.
*   **Prompting:** Los prompts deben ser específicos, indicando el archivo objetivo y las reglas de Rust a seguir (ej: "No uses unwrap").
*   **Feedback:** Si un agente falla, se debe proporcionar el error de compilación exacto para que OpenCode pueda corregirlo.

---

## 4. Gestión de Riesgos en la IA

*   **Alucinaciones:** Antigravity debe verificar que las librerías propuestas existen y son compatibles con Rust actual.
*   **Código Verboso:** Antigravity debe instar a OpenCode a mantener funciones pequeñas y modulares.
*   **Seguridad:** El Security Agent debe revisar proactivamente el código generado por OpenCode en áreas críticas (Auth, DB).
