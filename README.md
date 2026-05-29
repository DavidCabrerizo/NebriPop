# NebriPop — Marketplace Universitario en Rust

NebriPop es una aplicación web tipo Wallapop orientada a la compraventa de productos de segunda mano entre estudiantes, profesores y miembros de la comunidad universitaria.

## 🚀 Estado del Proyecto
**Estado Actual:** MVP Finalizado (Fase 10 completada) ✅.

Este proyecto destaca por el uso de **Rust de extremo a extremo**:

*   **Backend:** [Axum](https://github.com/tokio-rs/axum) + [SQLx](https://github.com/launchbadge/sqlx) + [SQLite](https://www.sqlite.org/).
*   **Frontend:** [Leptos](https://leptos.dev/) (Rust Full-stack).
*   **Desarrollo Asistido:** Coordinado íntegramente mediante un sistema multi-agente de Inteligencia Artificial (**Antigravity**, **OpenCode**, y roles especializados).

## 📂 Estructura del Proyecto

*   `backend/`: API REST desarrollada con Axum. Revisa el [README del Backend](./backend/README.md).
*   `frontend/`: Interfaz de usuario reactiva con Leptos. Revisa el [README del Frontend](./frontend/README.md).
*   `database/`: Scripts de inicialización y base de datos embebida SQLite. Revisa el [README de Base de Datos](./database/README.md).
*   `docs/`: Documentación técnica, PRD y planes de implementación.

## 📜 Documentación de Gobernanza y Final
*   [Memoria del Proyecto](./docs/MEMORIA_PROYECTO.md): Informe académico y de arquitectura final.
*   [Manual de Usuario](./docs/USER_GUIDE.md): Guía de uso de la aplicación.
*   [Informe MVP](./docs/FINAL_REPORT.md): Conclusiones técnicas de la Fase 10.
*   [AGENTS.md](./AGENTS.md): Definición del equipo de agentes IA.
*   [RULES.md](./RULES.md): Reglas obligatorias de desarrollo y seguridad.

## 🛠️ Requisitos previos

*   [Rust](https://www.rust-lang.org/tools/install) (última versión estable).
*   Trunk (`cargo install trunk`) para servir el frontend.

## ⚙️ Instalación y Uso (Modo Desarrollo)

1. **Clonar repositorio:**
   ```bash
   git clone https://github.com/DavidCabrerizo/NebriPop.git
   cd NebriPop
   ```
2. **Configurar el entorno:**
   Copia `.env.example` a `.env` en la raíz del proyecto.
   ```bash
   cp .env.example .env
   ```
3. **Iniciar el Backend:**
   Abre una terminal y ejecuta:
   ```bash
   cd backend
   cargo run
   ```
4. **Iniciar el Frontend:**
   Abre otra terminal y ejecuta:
   ```bash
   cd frontend
   trunk serve --open
   ```

---
Desarrollado con ❤️ mediante Inteligencia Artificial y Rust.
