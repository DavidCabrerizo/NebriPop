# Plan de Implementación — NebriPop

Este documento detalla la hoja de ruta técnica para el desarrollo de **NebriPop**, un marketplace académico para la comunidad universitaria, desarrollado íntegramente con **Rust** (Frontend y Backend) y asistido por Inteligencia Artificial (**Antigravity** y **OpenCode**).

---

## 1. Resumen técnico del producto
NebriPop es una plataforma de compraventa de artículos de segunda mano optimizada para el entorno universitario.

*   **Qué se va a construir:** Una Single Page Application (SPA) con frontend en Rust (Leptos) y una API REST en Rust (Axum).
*   **Usuarios principales:** Estudiantes, profesores y personal de la comunidad Nebrija.
*   **Funcionalidades principales:** Autenticación JWT, CRUD de productos, sistema de búsqueda y filtrado por categorías, gestión de favoritos y mensajería básica entre usuarios.
*   **Alcance del MVP:** Flujo completo de registro/login, publicación de anuncios con imagen, buscador funcional, gestión de perfil y contacto entre comprador/vendedor.
*   **Funcionalidades futuras:** Chat en tiempo real (WebSockets), pagos integrados, valoraciones, geolocalización y verificación institucional.

---

## 2. Stack tecnológico propuesto
Se ha seleccionado un stack basado en la seguridad, el rendimiento y la coherencia del lenguaje (Rust de extremo a extremo).

*   **Backend (Rust):** **Axum** como framework web por su integración con el ecosistema **Tokio**.
*   **Frontend (Rust):** **Leptos** (arquitectura de señales) para una UI reactiva y eficiente.
*   **Base de datos:** **PostgreSQL** para persistencia robusta y relacional.
*   **Capa de acceso a datos:** **SQLx** para consultas seguras en tiempo de compilación y migraciones asíncronas.
*   **Autenticación:** **JWT** (JSON Web Tokens) y cifrado de contraseñas con **Argon2id**.
*   **Gestión de imágenes:** Almacenamiento en sistema de archivos local (volumen Docker) referenciado por URL en la BD (MVP).
*   **Testing:** `cargo test` para unitarios e integración; **Tower-test** para pruebas de API.
*   **Documentación API:** **Utoipa** (OpenAPI/Swagger) para generación automática de docs.
*   **Despliegue:** **Docker** y **Docker Compose** para orquestar App + DB.
*   **CI/CD:** **GitHub Actions** para validación de compilación, tests y linting.

---

## 3. Arquitectura del sistema
El sistema sigue una arquitectura de **N-Capas** desacopladas.

*   **Separación Frontend/Backend:** Comunicación mediante API REST (JSON).
*   **Estructura del Backend:**
    *   `routes/`: Definición de rutas y mapeo de endpoints.
    *   `handlers/`: Orquestación de peticiones, validación de entrada (DTO) y respuestas.
    *   `services/`: Lógica de negocio pura, agnóstica de la infraestructura.
    *   `repositories/`: Interacción directa con SQLx y PostgreSQL.
    *   `models/`: Estructuras de dominio.
    *   `dto/`: Data Transfer Objects para entrada/salida de la API.
*   **Patrón Arquitectónico:** **Domain-Driven Design (DDD) simplificado** con patrón **Repository**.
*   **Gestión de Errores:** Enum centralizado de errores con implementación de `IntoResponse` para Axum para devolver códigos HTTP coherentes.
*   **Seguridad:** Middleware de autenticación JWT, validación estricta de esquemas (Serde) y headers de seguridad básica (CORS configurado).

---

## 4. Modelo de datos
Entidades principales diseñadas para PostgreSQL.

### Usuario (users)
*   `id`: UUID (PK)
*   `full_name`: VARCHAR(100)
*   `email`: VARCHAR(255) (Unique, Index)
*   `password_hash`: TEXT
*   `role`: Enum (user, admin)
*   `is_active`: BOOLEAN (Default true)
*   `created_at`: TIMESTAMP

### Categoría (categories)
*   `id`: SERIAL (PK)
*   `name`: VARCHAR(50) (Unique)
*   `slug`: VARCHAR(50) (Unique)

### Producto (products)
*   `id`: UUID (PK)
*   `seller_id`: UUID (FK -> users.id)
*   `category_id`: INTEGER (FK -> categories.id)
*   `title`: VARCHAR(100) (Index para búsqueda)
*   `description`: TEXT
*   `price`: DECIMAL(10, 2)
*   `status`: Enum (available, reserved, sold, deleted)
*   `location`: VARCHAR(100)
*   `is_active`: BOOLEAN (Default true)
*   `created_at`: TIMESTAMP

### Imagen (product_images)
*   `id`: UUID (PK)
*   `product_id`: UUID (FK -> products.id)
*   `url`: TEXT
*   `is_primary`: BOOLEAN

### Favorito (favorites)
*   `user_id`: UUID (FK -> users.id)
*   `product_id`: UUID (FK -> products.id)
*   PK (user_id, product_id)

### Mensaje (messages)
*   `id`: UUID (PK)
*   `sender_id`: UUID (FK -> users.id)
*   `receiver_id`: UUID (FK -> users.id)
*   `product_id`: UUID (FK -> products.id)
*   `content`: TEXT
*   `created_at`: TIMESTAMP

---

## 5. Endpoints de la API (Propuesta inicial)

| Categoría | Método | Ruta | Descripción | Prioridad |
| :--- | :--- | :--- | :--- | :--- |
| **Auth** | POST | `/api/auth/register` | Registro de nuevo usuario | MVP |
| **Auth** | POST | `/api/auth/login` | Login y retorno de JWT | MVP |
| **User** | GET | `/api/users/me` | Obtener perfil actual | MVP |
| **Prod** | GET | `/api/products` | Listar productos (con filtros/búsqueda) | MVP |
| **Prod** | POST | `/api/products` | Publicar nuevo producto (Auth) | MVP |
| **Prod** | GET | `/api/products/{id}` | Detalle de un producto | MVP |
| **Prod** | PUT | `/api/products/{id}` | Editar producto propio (Auth) | MVP |
| **Prod** | DELETE | `/api/products/{id}` | Eliminar producto propio (Auth) | MVP |
| **Cat** | GET | `/api/categories` | Listar todas las categorías | MVP |
| **Fav** | POST | `/api/favorites/{pid}` | Añadir/quitar de favoritos | MVP |
| **Msg** | POST | `/api/messages` | Enviar mensaje inicial | MVP |
| **Msg** | GET | `/api/messages/chat/{id}` | Ver hilo de conversación | MVP |
| **Admin** | GET | `/api/admin/products` | Listado total para moderación (Admin) | MVP |
| **Admin** | DELETE | `/api/admin/products/{id}` | Borrado lógico de producto (Admin) | MVP |

---

## 6. Plan de implementación por fases

### Fase 0: Preparación del repositorio
*   **Objetivo:** Establecer la base del proyecto y el entorno de agentes.
*   **Tareas:** `cargo new` (workspace), configuración de `.gitignore`, `docker-compose.yml` inicial (DB).
*   **Agente:** Software Architect Agent (Antigravity).

### Fase 1: Backend base y Estructura
*   **Objetivo:** Setup de Axum y gestión de errores.
*   **Tareas:** Configurar servidor Axum, Graceful shutdown, Logger, Middleware de errores, Política CORS explícita.
*   **Agente:** Rust Backend Architect Agent (OpenCode).

### Fase 2: Base de Datos y Modelos
*   **Objetivo:** Persistencia de datos.
*   **Tareas:** Migraciones SQLx, Repositorios base para Users y Products.
*   **Agente:** Database Agent (OpenCode).

### Fase 3: Autenticación
*   **Objetivo:** Seguridad de la plataforma.
*   **Tareas:** Implementar registro (Validación dominio @nebrija.es), login (Argon2), generación/validación JWT y middleware `AuthExt`.
*   **Agente:** Security Agent & Backend Agent (OpenCode).

### Fase 4: CRUD de Productos y Búsqueda
*   **Objetivo:** Funcionalidad core de negocio.
*   **Tareas:** Endpoints de creación, listado con filtros SQL y búsqueda por texto (ILIKE o FTS).
*   **Agente:** Rust Backend Agent (OpenCode).

### Fase 5: Frontend base (Leptos)
*   **Objetivo:** Skeleton de la UI.
*   **Tareas:** Configurar Leptos CSR, Layout (Navbar/Footer), Rutas de navegación.
*   **Agente:** Rust Frontend Agent (Antigravity/OpenCode).

### Fase 6: Integración y Páginas Core
*   **Objetivo:** Conexión completa.
*   **Tareas:** Consumo de API en login, registro y catálogo. Gestión de estado del JWT en el cliente.
*   **Agente:** Rust Frontend Agent (OpenCode).

---

## 7. Agentes necesarios (Antigravity)

1.  **Product Manager Agent:** Valida que cada implementación cumpla con el PRD.md. (Modelo: Gemini 1.5 Pro).
2.  **Rust Backend Architect Agent:** Diseña los contratos de la API y la estructura de servicios. (Modelo: Gemini 1.5 Pro).
3.  **Rust Frontend Agent:** Implementa la UI reactiva con Leptos. (Modelo: Gemini 1.5 Pro).
4.  **Database Agent:** Optimiza queries SQL y gestiona migraciones. (Modelo: Gemini 1.5 Flash).
5.  **QA & Testing Agent:** Genera unit tests y planes de prueba de integración. (Modelo: Gemini 1.5 Flash).
6.  **Security Agent:** Audita el manejo de JWT y los permisos de usuario. (Modelo: Gemini 1.5 Pro).
7.  **DevOps Agent:** Gestiona Dockerfiles y pipelines de CI/CD. (Modelo: Gemini 1.5 Flash).
8.  **Documentation Agent:** Mantiene actualizados los archivos `.md` y la doc de la API. (Modelo: Gemini 1.5 Flash).

---

## 8. Flujo de trabajo entre Antigravity y OpenCode

1.  **Antigravity (Cerebro):** Se encarga de la **Planificación** y **Arquitectura**. Define *qué* hacer, crea el `implementation_plan.md` y divide las tareas en el `task.md`.
2.  **OpenCode (Mano Ejecutora):** Recibe las instrucciones detalladas de Antigravity. Genera el código Rust, implementa los tests y corrige errores de compilación.
3.  **Ciclo de Validación:**
    *   Antigravity propone un diseño de módulo.
    *   OpenCode lo implementa.
    *   Antigravity revisa el código generado para asegurar que sigue los principios SOLID.
    *   OpenCode refina el código según el feedback.

---

## 9. Prompts base para ejecución

### Crear estructura inicial (Antigravity)
> "Actúa como Rust Architect. Inicializa un workspace de Rust para NebriPop con dos miembros: `backend` (Axum) y `frontend` (Leptos). Configura un `docker-compose.yml` que incluya PostgreSQL y la estructura de carpetas definida en el PRD.md."

### Implementar Modelos (OpenCode)
> "Genera las entidades de base de datos en Rust usando SQLx para las tablas `users`, `products` y `categories` según el diseño del IMPLEMENTATION_PLAN.md. Incluye los DTOs de Serde para entrada y salida de la API."

### Crear Endpoints CRUD (OpenCode)
> "Implementa los handlers de Axum para el CRUD de productos. Asegúrate de que el middleware de autenticación valide que solo el propietario puede editar o eliminar sus anuncios. Usa el patrón Repository."

---

## 10. Reglas de desarrollo

1.  **Strict Typing:** Prohibido el uso de `unwrap()` en código de producción; gestionar todos los `Result` y `Option`.
2.  **Clean Code:** Nombres de funciones y variables descriptivos en inglés (código) pero comentarios/docs pueden estar en español.
3.  **No Mutation without Reason:** Preferir inmutabilidad y paso por referencia.
4.  **Ownership Rules:** Respetar los principios de Rust; evitar clones innecesarios.
5.  **Audit Log:** Antes de cada fase, el agente debe actualizar el `task.md`.
6.  **Review Mandatory:** OpenCode no puede dar por finalizada una tarea si el código no pasa `cargo clippy`.

---

## 11. Testing y QA
*   **Unit Tests:** Pruebas de lógica de negocio en `services/`.
*   **Integration Tests:** Pruebas de endpoints usando `axum::Router` y una BD de test.
*   **UI Tests:** Verificación manual de flujos críticos (registro -> publicar -> buscar).
*   **Checklist MVP:**
    *   [ ] El usuario puede registrarse y loguearse.
    *   [ ] Un producto publicado aparece en el catálogo.
    *   [ ] El buscador filtra correctamente por texto y categoría.
    *   [ ] El botón de favorito persiste el estado en la BD.

---

## 12. CI/CD y despliegue
*   **Control de Versiones:** Main (protegida), Develop (integración), Feature branches.
*   **CI:** GitHub Action que ejecute `cargo check`, `cargo fmt --check`, `cargo clippy` y `cargo test`.
*   **CD:** Dockerización de la app para despliegue en VPS o plataforma cloud compatible con contenedores.

---

## 13. Documentación obligatoria
*   `ARCHITECTURE.md`: Diagramas de flujo y detalle de componentes.
*   `API_DOCS.md`: Lista de endpoints (o enlace a Swagger si se implementa Utoipa).
*   `DATABASE_SCHEMA.md`: Diagrama ER y descripción de tablas.
*   `PROMPTS_LOG.md`: Historial de prompts clave usados con los agentes.

---

## 14. Roadmap Final (Estimado)

| Fase | Prioridad | Duración | Responsable | Resultado esperado |
| :--- | :--- | :--- | :--- | :--- |
| **F0-F2** | Crítica | 3 días | Architect / DB Agent | Proyecto base + DB lista |
| **F3-F4** | Crítica | 5 días | Backend / Security Agent | API funcional de productos |
| **F5-F6** | Alta | 5 días | Frontend Agent | Web funcional conectada |
| **F7-F8** | Media | 2 días | QA / DevOps Agent | Proyecto estable y dockerizado |

---

## Preguntas abiertas
1.  **Almacenamiento de Imágenes:** ¿Se prefiere almacenamiento local en volumen Docker para el MVP o integración directa con un servicio tipo Cloudinary desde el inicio?
2.  **Validación de Email:** ¿Es necesaria la verificación real de email para el MVP o basta con la restricción de dominio `@nebrija.es`?
3.  **Estado del Chat:** ¿El "contacto básico" del MVP implica un sistema de mensajes persistentes o simplemente un enlace de "Enviar consulta" que notifica al vendedor?
