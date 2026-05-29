# Memoria Académica del Proyecto: NebriPop

## 1. Introducción
NebriPop es un proyecto académico de ingeniería de software centrado en la creación de una plataforma web de compraventa (marketplace) destinada a la comunidad universitaria. Surge con la premisa de fomentar la economía circular entre estudiantes (libros, material técnico, alojamiento, etc.) facilitando el contacto de una manera rápida y segura.

## 2. Objetivos y Justificación
El objetivo primordial de este proyecto fue validar la viabilidad y robustez del lenguaje **Rust** tanto en el desarrollo de la interfaz de usuario web (Frontend) como en el servidor de APIs (Backend). A nivel funcional, el proyecto justifica su existencia por la necesidad de una plataforma segura y cerrada, de contexto universitario, que supere los roces de las aplicaciones comerciales masivas de segunda mano.

## 3. Marco Tecnológico
- **Lenguaje Principal:** Rust (Seguridad de memoria, alto rendimiento).
- **Backend:** Axum (Framework web asíncrono sobre el ecosistema Tokio).
- **Capa de Persistencia:** SQLx conectando a una base de datos embebida **SQLite** (elegida por su ligereza y simplicidad de entrega para el MVP).
- **Frontend:** Leptos (Framework reactivo basado en señales finas) renderizado del lado del cliente (CSR).
- **Autenticación:** JSON Web Tokens (JWT) y Hash de contraseñas mediante bcrypt.

## 4. Metodología y Uso de Inteligencia Artificial
La característica más innovadora del proyecto ha sido su metodología de desarrollo colaborativo multi-agente:
- Se implementó un entorno coordinado de agentes IA (Product Manager, UXDesigner, SoftwareArchitect, Frontend y Backend Agents, Reviewer, QA, etc.).
- La toma de decisiones, arquitectura y validaciones pasaron por "puertas de revisión" de IA antes de que los modelos generadores produjeran el código.
- Este enfoque demostró acelerar drásticamente los tiempos de andamiaje y la resolución de problemas (por ejemplo, correcciones de CORS, enrutamiento tipado, y ownership en Rust).

## 5. Arquitectura del Sistema
La plataforma se dividió usando una arquitectura en capas:
- **Backend:** Dividido en *Routes* (Endpoint mapping), *Handlers* (Validación HTTP y extracciones), *Services* (Lógica de negocio aislada) y *Repositories* (Aislamiento de la base de datos).
- **Frontend:** Single Page Application modularizada en *Pages* (vistas principales), *Components* (elementos reusables como layouts, cards, inputs), *API* (clientes HTTP), y *Models* (definición de estructuras).
- El contrato de comunicación cliente/servidor estuvo fuertemente tipado gracias a Serde, serializando el tráfico en formato JSON.

## 6. Diseño y Experiencia de Usuario (UX/UI)
La UI se conceptualizó buscando similitud con aplicaciones comerciales para reducir la curva de aprendizaje:
- Tema claro/oscuro (preparado).
- Navegación lateral y superior intuitiva con diseño responsivo.
- Sistema de interacciones de alta retroalimentación gracias a las señales de Leptos, permitiendo filtros sin recarga de la página y previsualización instatánea.

## 7. Implementación de Funcionalidades (MVP)
El sistema culmina con la implementación exitosa de:
1. **Sistema de Cuentas:** Registro, cifrado y JWT en cabecera de autenticación.
2. **Catálogo Reactivo:** CRUD completo de productos con subida de imágenes, categorías integradas dinámicamente y filtros reactivos.
3. **Interacción:** Funcionalidades de "Me gusta" (Favoritos) y "Mensajería" que conectan al vendedor y al comprador mediante una tabla relacional unificada en SQLite.

## 8. Conclusiones y Trabajo Futuro
El uso de Rust en todo el ciclo de desarrollo web (Full-Stack) supone una curva de aprendizaje elevada debido al estricto chequeo del compilador y el manejo de tiempos de vida (*lifetimes*). Sin embargo, una vez compilado, elimina categorías enteras de errores en tiempo de ejecución (ej. punteros nulos, data races).

Como **Trabajo Futuro**, se postula la migración de SQLite a PostgreSQL (la arquitectura de Repository lo permite sin gran fricción), la adición de WebSockets para chats instantáneos, y sistemas automáticos de recomendación académica.
