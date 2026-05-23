# RULES — NebriPop Development Guidelines

Este documento establece las reglas obligatorias para todo el desarrollo del proyecto NebriPop. Estas reglas deben ser respetadas por todos los agentes (Antigravity y OpenCode).

---

## 1. Reglas Generales de Rust
1.  **Strict Typing:** Prohibido el uso de `.unwrap()` o `.expect()` en código de producción. Se debe realizar una gestión de errores explícita usando `Result` y `Option`.
2.  **Safety:** No se permite el uso de bloques `unsafe` a menos que sea estrictamente necesario y esté justificado por el Software Architect.
3.  **Ownership:** Seguir los principios de borrow checker. Evitar clones innecesarios (`.clone()`) y preferir el paso de referencias.
4.  **Linting:** El código debe pasar `cargo fmt` y `cargo clippy` sin warnings críticos antes de considerarse terminado.

---

## 2. Arquitectura y Código Limpio (Clean Code)
1.  **Nomenclatura:**
    *   Código (variables, funciones, archivos): **Inglés**.
    *   Documentación y comentarios: **Español**.
    *   Casos: `snake_case` para funciones/variables, `PascalCase` para tipos/structs.
2.  **Separación de Responsabilidades:**
    *   **Backend:** Respetar la estructura `Routes -> Handlers -> Services -> Repositories`.
    *   **Frontend:** Separar componentes de UI de la lógica de servicios de API.
3.  **Inmutabilidad:** Preferir variables inmutables por defecto.
4.  **Funciones Pequeñas:** Cada función debe realizar una única tarea bien definida (Single Responsibility Principle).

---

## 3. Seguridad
1.  **Secretos:** Prohibido subir archivos `.env` o secretos al repositorio. Usar `.env.example`.
2.  **Passwords:** Nunca almacenar contraseñas en texto plano. Usar **Argon2id**.
3.  **Autenticación:** Todas las rutas sensibles deben estar protegidas por el middleware de JWT.
4.  **Validación de Entrada:** Validar todos los datos recibidos desde el exterior (API o Formularios) utilizando las capacidades de tipado de Rust y validaciones personalizadas.

---

## 4. Base de Datos
1.  **Migraciones:** No se permiten cambios manuales en la base de datos. Todo cambio debe realizarse mediante migraciones de SQLx.
2.  **Borrado Lógico:** No eliminar registros de `users` o `products` físicamente. Usar el campo `is_active` o estados específicos (`deleted`).
3.  **Tipado:** Usar `UUID` para identificadores de claves primarias en tablas expuestas al público.

---

## 5. Gestión de IA (Antigravity & OpenCode)
1.  **Trazabilidad:** Los prompts más relevantes deben registrarse en `PROMPTS_LOG.md`.
2.  **Registro de Logs (Automático):** Es obligatorio que cualquier cambio o tarea finalizada quede reflejada en `docs/Logs.md`. Esta actualización se realizará de forma automática tras cada modificación, sin necesidad de solicitar autorización previa al usuario.
3.  **Validación PM:** Antes de iniciar una nueva fase, el Project Manager Agent debe validar que la fase anterior cumple con los criterios de aceptación.
3.  **Task Management:** El archivo `task.md` debe ser la única fuente de verdad sobre el estado actual de las tareas.

---

## 6. Frontend (Leptos)
1.  **Señales:** Usar el sistema de reactividad de Leptos (signals) de forma eficiente.
2.  **Componentes:** Dividir la UI en componentes reutilizables y modulares.
3.  **Responsividad:** Seguir un enfoque "Mobile First" en el diseño CSS.
