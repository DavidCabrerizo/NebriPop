# Informe Final del MVP (NebriPop)

## 1. Resumen Ejecutivo
Este informe detalla el estado final del Producto Mínimo Viable (MVP) de **NebriPop**, un marketplace académico de segunda mano. El proyecto ha sido desarrollado siguiendo una arquitectura moderna full-stack utilizando **Rust** en ambos extremos (Axum y Leptos) y ha sido asistido por sistemas multi-agente de Inteligencia Artificial. 

El MVP es totalmente funcional y cumple con el core de negocio establecido en el `PRD.md`.

## 2. Alcance Evaluado (Características MVP)
Las siguientes funcionalidades han sido implementadas, probadas y consolidadas:
- **Gestión de Usuarios:** Registro seguro con hashing de contraseñas (bcrypt), inicio de sesión y autenticación vía JWT (JSON Web Tokens).
- **Gestión de Productos:** CRUD completo. Los usuarios pueden publicar, ver, buscar, filtrar, editar (todos los campos) y borrar productos.
- **Gestión de Imágenes:** Sistema nativo de subida y eliminación de imágenes integradas localmente.
- **Búsqueda y Filtros:** Implementados en tiempo real mediante *signals* de Leptos en UI y *Query Builders* en Axum, incluyendo filtros de categoría, precio, y estado.
- **Interacción Social:** Sistema de favoritos (guardar/eliminar) y un sistema de mensajería asíncrona (inbox y chat uno-a-uno) que permite conectar al comprador con el vendedor.

## 3. Decisiones Arquitectónicas y Tecnológicas Finales
- **Backend:** Axum y Tokio para concurrencia masiva.
- **Frontend:** Leptos con CSR (Client-Side Rendering) posibilitando una auténtica Single Page Application altamente reactiva.
- **Base de Datos:** Inicialmente pensada para PostgreSQL, la arquitectura se pivoteó a **SQLite** para minimizar la carga de DevOps durante el desarrollo del MVP y simplificar el despliegue a los usuarios, manteniendo compatibilidad transaccional a través de `sqlx`.
- **Seguridad:** Consultas parametrizadas anti-SQLi, control de acceso perimetral (CORS), y cifrado de datos críticos.

## 4. Requisitos No Funcionales Cumplidos
- **Rendimiento:** Tiempos de compilación de Rust amortizados por la velocidad de ejecución y un binario final eficiente.
- **Usabilidad:** Interfaz unificada, similar a marketplaces líderes en el mercado, manteniendo estética académica.
- **Despliegue:** Sistema contenido sin dependencias pesadas de contenedores DB de terceros gracias a SQLite.

## 5. Funcionalidades Futuras (Fuera del Alcance del MVP)
Para futuras versiones (v2.0+), quedan pendientes:
- Sistema de mensajería en tiempo real usando WebSockets.
- Integración con pasarelas de pago.
- Sistema de puntuación/rating de perfiles.
- Moderación automática asistida por IA.

## 6. Conclusión
NebriPop ha alcanzado exitosamente su estado de MVP. La base de código es sólida, modular y escalable, sentando una excelente base para un crecimiento a escala empresarial si fuera requerido en el futuro.
