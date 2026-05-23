PRD — NebriPop
1. Breve descripción de la aplicación

NebriPop es una aplicación web tipo Wallapop orientada a la compraventa de productos de segunda mano entre estudiantes, profesores y miembros de la comunidad universitaria.

La aplicación permitirá a los usuarios registrarse, iniciar sesión, publicar productos, buscar artículos, filtrar por categorías, guardar favoritos y contactar con otros usuarios interesados en comprar o vender productos.

El proyecto se desarrollará utilizando Rust tanto en el frontend como en el backend, apoyándose en herramientas de inteligencia artificial como Antigravity y OpenCode para la planificación, generación de código, revisión, testing, documentación y despliegue.

2. Objetivos
Objetivo principal

Desarrollar una aplicación funcional de marketplace llamada NebriPop, aplicando buenas prácticas de ingeniería del software y utilizando herramientas de IA de forma estructurada y documentada.

Objetivos específicos
Crear una plataforma sencilla y segura para publicar y comprar productos de segunda mano.
Implementar una arquitectura limpia y escalable.
Utilizar Rust en frontend y backend.
Aplicar principios de Clean Code, SOLID y patrones de diseño.
Documentar todo el proceso de desarrollo asistido por IA.
Utilizar Antigravity para planificación, arquitectura y coordinación.
Utilizar OpenCode para implementación, refactorización, testing y corrección de errores.
Crear una aplicación desplegable mediante Docker.
Implementar pruebas de calidad y control del estado del proyecto.
3. Público objetivo

El público objetivo de NebriPop está formado por usuarios relacionados con el entorno universitario.

Usuarios principales
Estudiantes universitarios.
Profesores.
Personal administrativo.
Antiguos alumnos.
Personas vinculadas a la comunidad Nebrija.
Necesidades del usuario
Comprar productos de segunda mano de forma rápida.
Vender objetos que ya no usan.
Encontrar productos cercanos o relacionados con el entorno universitario.
Contactar fácilmente con otros usuarios.
Ahorrar dinero en productos como libros, apuntes, tecnología, material académico, muebles o transporte.
Ejemplos de usuarios
Usuario	Necesidad
Estudiante de primer año	Comprar libros de segunda mano
Estudiante que se muda	Vender muebles o pequeños electrodomésticos
Alumno de informática	Comprar portátil, periféricos o material técnico
Profesor	Publicar libros, material académico o recursos
4. Scope del proyecto
4.1 MVP

El MVP será la primera versión funcional de NebriPop. Incluirá las funcionalidades mínimas necesarias para que la aplicación pueda ser utilizada como marketplace básico.

Funcionalidades incluidas en el MVP
Registro de usuarios.
Inicio de sesión.
Cierre de sesión.
Publicación de productos.
Listado de productos.
Búsqueda de productos.
Filtro por categoría.
Vista de detalle de producto.
Edición de productos propios.
Eliminación de productos propios.
Estado del producto: disponible, reservado o vendido.
Sistema de favoritos.
Contacto básico con el vendedor.
Perfil de usuario.
Panel básico de administración.
Diseño responsive.
API REST.
Base de datos relacional.
4.2 Funcionalidades fuera del MVP

Estas funcionalidades podrían añadirse en versiones futuras, pero no son obligatorias para la primera entrega.

Chat en tiempo real.
Sistema de pagos integrado.
Valoraciones entre usuarios.
Geolocalización avanzada.
Notificaciones push.
Moderación automática con IA.
Recomendador de productos.
Subida múltiple avanzada de imágenes.
Sistema de reportes.
Verificación universitaria mediante correo institucional.
Aplicación móvil.
5. Casos de uso
Caso de uso 1 — Registro de usuario
Descripción

Un usuario nuevo quiere crear una cuenta para poder comprar o vender productos.

Actor principal

Usuario no registrado.

Flujo principal
El usuario accede a la página de registro.
Introduce nombre, email y contraseña.
El sistema valida los datos.
El sistema guarda el usuario con la contraseña cifrada.
El sistema muestra confirmación o inicia sesión automáticamente.
Ejemplo

Un estudiante crea una cuenta con su correo para poder vender libros usados.

Caso de uso 2 — Inicio de sesión
Descripción

Un usuario registrado quiere acceder a su cuenta.

Actor principal

Usuario registrado.

Flujo principal
El usuario introduce email y contraseña.
El sistema valida las credenciales.
El sistema genera un token de autenticación.
El usuario accede a su panel personal.
Caso de uso 3 — Publicar producto
Descripción

Un usuario quiere publicar un producto para venderlo.

Actor principal

Usuario autenticado.

Flujo principal
El usuario accede a “Publicar producto”.
Introduce título, descripción, precio, categoría, ubicación e imagen.
El sistema valida los datos.
El producto queda publicado como “disponible”.
El producto aparece en el catálogo.
Ejemplo

Un alumno publica una calculadora científica por 20 euros.

Caso de uso 4 — Buscar producto
Descripción

Un usuario quiere encontrar productos dentro de la plataforma.

Actor principal

Usuario visitante o autenticado.

Flujo principal
El usuario entra en el catálogo.
Introduce una palabra clave.
Puede aplicar filtros por categoría, precio o estado.
El sistema devuelve los productos coincidentes.
Ejemplo

Un usuario busca “libro programación” y filtra por categoría “Libros”.

Caso de uso 5 — Ver detalle de producto
Descripción

Un usuario quiere consultar información completa de un producto.

Actor principal

Usuario visitante o autenticado.

Flujo principal
El usuario selecciona un producto.
El sistema muestra título, descripción, precio, imagen, vendedor y estado.
Si el usuario está autenticado, puede marcar favorito o contactar con el vendedor.
Caso de uso 6 — Contactar con vendedor
Descripción

Un usuario quiere contactar con el vendedor para preguntar por un producto.

Actor principal

Usuario autenticado.

Flujo principal
El usuario pulsa “Contactar”.
Escribe un mensaje.
El sistema guarda el mensaje.
El vendedor puede consultar los mensajes recibidos.
Caso de uso 7 — Marcar producto como favorito
Descripción

Un usuario quiere guardar productos para verlos más tarde.

Actor principal

Usuario autenticado.

Flujo principal
El usuario pulsa el botón de favorito.
El sistema guarda la relación usuario-producto.
El producto aparece en la sección de favoritos.
Caso de uso 8 — Editar o eliminar producto
Descripción

Un usuario quiere modificar o borrar un producto publicado por él.

Actor principal

Usuario autenticado propietario del producto.

Flujo principal
El usuario accede a su perfil.
Selecciona uno de sus productos.
Edita los datos o elimina el anuncio.
El sistema comprueba que el usuario es propietario.
El sistema actualiza o elimina el producto.
6. Requisitos
6.a Requisitos funcionales
Usuarios y autenticación
RF01: El sistema debe permitir registrar usuarios.
RF02: El sistema debe permitir iniciar sesión.
RF03: El sistema debe permitir cerrar sesión.
RF04: El sistema debe guardar las contraseñas cifradas.
RF05: El sistema debe utilizar autenticación mediante token JWT.
RF06: El sistema debe permitir consultar el perfil del usuario autenticado.
Productos
RF07: El sistema debe permitir publicar productos.
RF08: El sistema debe permitir listar productos.
RF09: El sistema debe permitir buscar productos por texto.
RF10: El sistema debe permitir filtrar productos por categoría.
RF11: El sistema debe permitir ver el detalle de un producto.
RF12: El sistema debe permitir editar productos propios.
RF13: El sistema debe permitir eliminar productos propios.
RF14: El sistema debe permitir cambiar el estado del producto.
RF15: El sistema debe asociar cada producto a un usuario vendedor.
Categorías
RF16: El sistema debe permitir clasificar productos por categoría.
RF17: El sistema debe cargar categorías iniciales desde la base de datos.
Favoritos
RF18: El sistema debe permitir añadir productos a favoritos.
RF19: El sistema debe permitir eliminar productos de favoritos.
RF20: El sistema debe permitir consultar los favoritos del usuario.
Mensajes
RF21: El sistema debe permitir enviar mensajes al vendedor.
RF22: El sistema debe permitir consultar conversaciones básicas.
RF23: El sistema debe vincular los mensajes a un producto.
Administración
RF24: El sistema debe incluir un rol de administrador.
RF25: El administrador debe poder revisar productos.
RF26: El administrador debe poder eliminar productos inadecuados.
Seguridad
RF27: Solo usuarios autenticados pueden publicar productos.
RF28: Solo el propietario puede editar o borrar sus productos.
RF29: Solo usuarios autenticados pueden enviar mensajes.
RF30: El sistema debe validar los datos recibidos en formularios y API.
6.b Requisitos no funcionales
Rendimiento
RNF01: La aplicación debe cargar el catálogo en un tiempo razonable.
RNF02: Las consultas principales deben estar optimizadas mediante índices en base de datos.
RNF03: El backend debe responder correctamente ante varias peticiones simultáneas.
Seguridad
RNF04: Las contraseñas deben almacenarse cifradas.
RNF05: No deben exponerse claves ni secretos en el repositorio.
RNF06: El sistema debe validar entradas para evitar errores o datos malformados.
RNF07: Las rutas protegidas deben requerir autenticación.
Usabilidad
RNF08: La interfaz debe ser sencilla e intuitiva.
RNF09: La aplicación debe ser responsive.
RNF10: Los formularios deben mostrar errores claros.
Mantenibilidad
RNF11: El código debe seguir una arquitectura modular.
RNF12: Se deben aplicar principios Clean Code.
RNF13: Se deben separar responsabilidades entre capas.
RNF14: El código debe estar documentado cuando sea necesario.
Escalabilidad
RNF15: La arquitectura debe permitir añadir nuevas funcionalidades.
RNF16: La base de datos debe estar normalizada.
RNF17: El backend debe poder ampliarse con nuevos módulos.
Calidad
RNF18: El proyecto debe incluir pruebas.
RNF19: El código debe pasar cargo fmt.
RNF20: El código debe pasar cargo clippy.
RNF21: El proyecto debe incluir integración continua.
7. UI/UX
Principios de diseño

La interfaz debe ser clara, moderna y parecida a un marketplace actual, pero manteniendo una estética sencilla y académica.

Objetivos de UI/UX
Facilitar la publicación de productos.
Facilitar la búsqueda.
Mostrar productos de forma visual.
Reducir pasos innecesarios.
Permitir uso cómodo desde móvil y escritorio.
Pantallas principales
Home

Debe incluir:

Nombre de la aplicación.
Buscador principal.
Productos destacados.
Categorías principales.
Botón para publicar producto.
Catálogo

Debe incluir:

Grid de productos.
Barra de búsqueda.
Filtros por categoría.
Filtro por precio.
Filtro por estado.
Cards de producto.
Detalle de producto

Debe incluir:

Imagen.
Título.
Precio.
Descripción.
Categoría.
Estado.
Vendedor.
Botón de favorito.
Botón de contactar.
Login

Debe incluir:

Email.
Contraseña.
Botón de iniciar sesión.
Enlace a registro.
Registro

Debe incluir:

Nombre.
Email.
Contraseña.
Confirmación de contraseña.
Botón de crear cuenta.
Publicar producto

Debe incluir:

Título.
Descripción.
Precio.
Categoría.
Ubicación.
Imagen.
Botón de publicar.
Perfil

Debe incluir:

Datos del usuario.
Productos publicados.
Favoritos.
Mensajes.
Panel admin

Debe incluir:

Listado de productos.
Listado de usuarios.
Acciones básicas de moderación.
8. Tecnologías y dependencias tecnológicas
Frontend
Rust
Leptos
HTML generado desde componentes Rust
CSS o framework de estilos compatible
Fetch/API client para comunicación con backend
Backend
Rust
Axum
Tokio
SQLx
Serde
JWT
Argon2 o bcrypt
UUID
dotenvy
Base de datos
PostgreSQL
Migraciones SQL
Datos seed iniciales
DevOps
Docker
Docker Compose
GitHub Actions
Variables de entorno
Testing
Cargo test
Tests unitarios
Tests de integración
QA manual
Clippy
Rustfmt
Herramientas IA
Antigravity
OpenCode
Herramientas de documentación y gestión
GitHub
Markdown
Notion o Trello, opcional
Slack o Discord, opcional
README
AGENTS.md
docs/
9. Timeline del proyecto
Fase 0 — Preparación

Duración estimada: 1 día.

Tareas:

Crear repositorio.
Crear estructura inicial.
Crear documentación base.
Definir agentes.
Crear AGENTS.md.

Entregables:

Repositorio inicial.
README.md.
docs/PRD.md.
docs/agentes.md.
Fase 1 — PRD y arquitectura

Duración estimada: 1 día.

Tareas:

Crear PRD con Antigravity.
Validar PRD con otro agente.
Diseñar arquitectura.
Definir base de datos.
Definir endpoints.

Entregables:

docs/PRD.md.
docs/arquitectura.md.
docs/metodologia.md.
docs/prompts.md.
Fase 2 — Backend base

Duración estimada: 2 días.

Tareas:

Crear proyecto backend con Axum.
Configurar PostgreSQL.
Crear health check.
Crear estructura por capas.
Crear gestión de errores.

Entregables:

Backend compilable.
Endpoint /health.
Conexión a base de datos.
Fase 3 — Autenticación y usuarios

Duración estimada: 2 días.

Tareas:

Registro.
Login.
JWT.
Middleware de autenticación.
Perfil de usuario.

Entregables:

Endpoints de autenticación.
Usuarios guardados en base de datos.
Rutas protegidas.
Fase 4 — Productos y catálogo

Duración estimada: 3 días.

Tareas:

Crear migraciones de productos.
Crear endpoints CRUD.
Crear filtros.
Crear búsqueda.
Validar permisos.

Entregables:

API de productos.
CRUD funcional.
Búsqueda básica.
Fase 5 — Frontend

Duración estimada: 3 días.

Tareas:

Crear frontend con Leptos.
Crear layout.
Crear páginas principales.
Crear componentes.
Conectar con API.

Entregables:

Interfaz funcional.
Login y registro conectados.
Catálogo conectado.
Publicación de productos conectada.
Fase 6 — Favoritos, mensajes y perfil

Duración estimada: 2 días.

Tareas:

Implementar favoritos.
Implementar mensajes básicos.
Crear perfil de usuario.
Mostrar productos propios.

Entregables:

Favoritos funcionales.
Contacto con vendedor.
Perfil de usuario.
Fase 7 — Testing y QA

Duración estimada: 2 días.

Tareas:

Crear tests backend.
Crear checklist UI.
Ejecutar Clippy.
Ejecutar Rustfmt.
Probar flujos principales.

Entregables:

docs/testing.md.
Tests ejecutados.
Informe QA.
Fase 8 — Docker, CI/CD y despliegue

Duración estimada: 2 días.

Tareas:

Crear Dockerfile.
Crear docker-compose.
Crear GitHub Actions.
Documentar despliegue.

Entregables:

docker-compose.yml.
.github/workflows/ci.yml.
docs/despliegue.md.
Fase 9 — Documentación final

Duración estimada: 1-2 días.

Tareas:

Completar memoria.
Documentar prompts.
Documentar agentes.
Documentar errores y soluciones.
Preparar exposición.

Entregables:

Memoria final.
Documentación IA.
Presentación del proyecto.
10. Métricas para evaluar el estado del proyecto — QA
Métricas funcionales
Métrica	Objetivo
Registro de usuario funcional	100%
Login funcional	100%
CRUD de productos funcional	100%
Búsqueda funcional	100%
Favoritos funcionales	100%
Mensajes básicos funcionales	100%
Perfil funcional	100%
Métricas técnicas
Métrica	Objetivo
Compilación backend	Sin errores
Compilación frontend	Sin errores
Cargo fmt	Sin errores
Cargo clippy	Sin warnings críticos
Tests unitarios	Mínimo 70% de los servicios principales
Tests integración	Endpoints principales cubiertos
Errores críticos abiertos	0 antes de entrega
Métricas de calidad
Métrica	Objetivo
Código modular	Sí
Separación de capas	Sí
Documentación actualizada	Sí
Prompts documentados	Sí
Agentes documentados	Sí
Arquitectura explicada	Sí
Seguridad básica aplicada	Sí
Métricas UX
Métrica	Objetivo
Flujo de registro claro	Sí
Publicar producto en menos de 5 pasos	Sí
Catálogo usable en móvil	Sí
Mensajes de error comprensibles	Sí
Diseño responsive	Sí
11. Riesgos
Riesgo 1 — Complejidad de Rust full-stack
Descripción

Rust es un lenguaje potente, pero más complejo que otros lenguajes web tradicionales.

Impacto

Alto.

Mitigación
Dividir el desarrollo en tareas pequeñas.
Usar agentes especializados.
Implementar primero un MVP sencillo.
Priorizar que compile y funcione antes de añadir extras.
Riesgo 2 — Exceso de alcance
Descripción

Intentar replicar demasiadas funcionalidades de Wallapop puede hacer que el proyecto no se termine.

Impacto

Alto.

Mitigación
Definir claramente el MVP.
Dejar funciones avanzadas como futuras mejoras.
Usar metodología incremental.
Riesgo 3 — Código generado por IA de baja calidad
Descripción

La IA puede generar código que compile parcialmente, tenga errores o no siga la arquitectura definida.

Impacto

Alto.

Mitigación
Crear AGENTS.md.
Usar prompts precisos.
Revisar cada cambio.
Ejecutar tests.
Usar Clippy y Rustfmt.
Documentar errores y correcciones.
Riesgo 4 — Mala integración frontend-backend
Descripción

El frontend y backend pueden no coincidir en rutas, DTOs o formato de respuestas.

Impacto

Medio-alto.

Mitigación
Definir contrato API antes de implementar.
Documentar endpoints.
Crear DTOs claros.
Probar cada endpoint con datos reales.
Riesgo 5 — Problemas con autenticación
Descripción

La gestión de JWT, permisos y rutas protegidas puede generar errores de seguridad.

Impacto

Alto.

Mitigación
Crear middleware específico.
Probar acceso autorizado y no autorizado.
Validar propiedad de productos.
No exponer tokens ni secretos.
Riesgo 6 — Falta de documentación IA
Descripción

El enunciado valora mucho la documentación del proceso con IA.

Impacto

Alto.

Mitigación
Guardar todos los prompts relevantes.
Documentar agentes.
Documentar decisiones.
Documentar errores.
Crear memoria de uso de IA.
Riesgo 7 — Despliegue incompleto
Descripción

Puede que la aplicación funcione localmente pero no se despliegue correctamente.

Impacto

Medio.

Mitigación
Usar Docker desde fases tempranas.
Crear .env.example.
Documentar pasos de despliegue.
Usar CI/CD básico.
12. Miembros: agentes y personas
Personas
Rol	Responsabilidad
Product Owner	Definir requisitos y alcance
Arquitecto software	Diseñar arquitectura y stack
Backend developer	Implementar API en Rust
Frontend developer	Implementar UI con Rust
QA tester	Probar funcionalidades
DevOps	Preparar Docker, CI/CD y despliegue
Documentador	Mantener memoria y evidencias

En caso de grupo reducido, una persona puede asumir varios roles.

Agentes IA
Product Owner Agent

Responsable de crear y validar el PRD.

Tareas:

Definir objetivos.
Definir usuarios.
Definir MVP.
Crear historias de usuario.
Definir criterios de aceptación.
Software Architect Agent

Responsable de la arquitectura.

Tareas:

Diseñar estructura del proyecto.
Definir capas.
Definir endpoints.
Definir base de datos.
Definir patrones de diseño.
Rust Backend Agent

Responsable del backend.

Tareas:

Implementar API con Axum.
Crear handlers, services y repositories.
Implementar autenticación.
Conectar PostgreSQL.
Crear tests backend.
Rust Frontend Agent

Responsable del frontend.

Tareas:

Crear interfaz con Leptos.
Crear componentes.
Crear páginas.
Conectar con API.
Mejorar UX.
Database Agent

Responsable del modelo de datos.

Tareas:

Crear tablas.
Crear migraciones.
Crear índices.
Crear seed de datos.
Revisar integridad.
QA Agent

Responsable de calidad.

Tareas:

Crear plan de pruebas.
Revisar requisitos.
Probar endpoints.
Probar UI.
Detectar errores.
DevOps Agent

Responsable de despliegue.

Tareas:

Crear Dockerfile.
Crear Docker Compose.
Crear CI/CD.
Documentar despliegue.
Documentation Agent

Responsable de documentación.

Tareas:

Guardar prompts.
Documentar decisiones.
Documentar errores.
Documentar evidencias.
Crear memoria final.
Security Agent

Responsable de seguridad.

Tareas:

Revisar autenticación.
Revisar permisos.
Revisar variables de entorno.
Revisar exposición de datos.
Revisar subida de imágenes.
13. Arquitectura, metodología y diseño software
Arquitectura general

NebriPop seguirá una arquitectura cliente-servidor.

Frontend Rust Leptos
        ↓
API REST Rust Axum
        ↓
Servicios de negocio
        ↓
Repositorios
        ↓
PostgreSQL
Arquitectura backend

El backend se organizará por capas:

routes/
handlers/
services/
repositories/
models/
dto/
auth/
errors/
config/
utils/
Routes

Definen las rutas HTTP.

Handlers

Reciben las peticiones, validan datos básicos y llaman a los servicios.

Services

Contienen la lógica de negocio.

Repositories

Gestionan el acceso a base de datos.

Models

Representan entidades internas.

DTO

Definen datos de entrada y salida.

Auth

Gestiona JWT, login, registro y middleware.

Errors

Centraliza errores de la API.

Arquitectura frontend

El frontend se organizará así:

components/
pages/
layouts/
services/
models/
utils/
styles/
Components

Elementos reutilizables como botones, cards, formularios y navbar.

Pages

Pantallas principales: home, catálogo, login, registro, perfil, detalle.

Services

Comunicación con la API.

Models

Tipos compartidos o equivalentes a los DTOs.

Modelo de base de datos

Tablas principales:

users
products
product_images
categories
favorites
messages

Relaciones:

Un usuario puede tener muchos productos.
Un producto pertenece a una categoría.
Un producto puede tener varias imágenes.
Un usuario puede tener muchos favoritos.
Un usuario puede enviar muchos mensajes.
Un mensaje puede estar asociado a un producto.
Metodología

Se utilizará una combinación de:

Extreme Programming

Por su enfoque en:

Iteraciones cortas.
Testing continuo.
Refactorización.
Feedback constante.
Código simple.
Mejora incremental.
Kanban

Para organizar tareas en columnas:

Backlog
To Do
In Progress
Review
Testing
Done

Esta metodología encaja bien con el uso de agentes IA, ya que cada tarea puede asignarse a un agente concreto.

Principios de diseño software
Clean Code
Nombres claros.
Funciones pequeñas.
Evitar duplicidad.
Separación de responsabilidades.
Código legible.
Comentarios solo cuando aporten valor.
SOLID
S: Cada módulo tiene una responsabilidad.
O: El sistema puede ampliarse sin modificar demasiado código existente.
L: Los componentes deben poder sustituirse respetando contratos.
I: Interfaces o contratos pequeños y específicos.
D: Las capas superiores no deben depender directamente de detalles de bajo nivel.
Patrones de diseño
Repository Pattern

Para separar la lógica de acceso a datos.

Service Layer

Para aislar la lógica de negocio.

DTO Pattern

Para separar modelos internos de datos enviados por la API.

Middleware Pattern

Para autenticación y autorización.

Factory simple

Para inicializar configuración, conexiones o servicios.

14. Utilidades
Guía de estilos
Estilo visual
Diseño limpio.
Colores principales: azul, blanco y gris claro.
Botones visibles.
Cards con bordes redondeados.
Tipografía legible.
Diseño responsive.
Tono de la aplicación
Cercano.
Universitario.
Sencillo.
Seguro.
Práctico.
Componentes UI
Navbar.
Footer.
Product Card.
Search Bar.
Category Filter.
Login Form.
Register Form.
Product Form.
Favorite Button.
Contact Button.
Admin Table.
Herramientas utilizadas
Desarrollo
Antigravity.
OpenCode.
Visual Studio Code o IDE compatible.
Rust.
Cargo.
Docker.
PostgreSQL.
Repositorio
Git.
GitHub.
Documentación
Markdown.
README.md.
docs/.
AGENTS.md.
Gestión del proyecto
Notion, Trello o GitHub Projects.
Kanban.
Issues.
Pull requests.
Comunicación
Slack, Discord o Microsoft Teams.
Testing y calidad
Cargo test.
Cargo fmt.
Cargo clippy.
GitHub Actions.
QA manual.
15. Cuestiones de negocio
Propuesta de valor

NebriPop permite a la comunidad universitaria comprar y vender productos de segunda mano en un entorno más cercano y específico que plataformas generalistas.

A diferencia de Wallapop, NebriPop estaría orientado a un entorno concreto: estudiantes y miembros de una universidad.

Problema detectado

Muchos estudiantes necesitan comprar o vender productos durante el curso académico, especialmente:

Libros.
Apuntes.
Material tecnológico.
Muebles.
Bicicletas o patinetes.
Material deportivo.
Ropa.
Pequeños electrodomésticos.

Las plataformas generalistas son útiles, pero no están especializadas en el entorno universitario.

Solución propuesta

Crear una plataforma específica para la comunidad Nebrija donde los usuarios puedan publicar, buscar y contactar de forma sencilla.

Estudio de mercado básico
Competidores
Competidor	Ventajas	Desventajas
Wallapop	Muy conocido, muchos usuarios	Demasiado generalista
Milanuncios	Mucho volumen	Interfaz menos moderna
Vinted	Especializado en moda	No sirve para todo tipo de productos
Facebook Marketplace	Integrado en red social	Menos controlado y menos específico
Diferenciación de NebriPop
Enfoque universitario.
Comunidad más cercana.
Productos útiles para estudiantes.
Experiencia más simple.
Posibilidad futura de verificación con correo universitario.
Menor distancia entre comprador y vendedor.
Modelo económico posible

En el MVP no se implementará monetización, pero se podrían estudiar varias opciones futuras:

Anuncios destacados.
Comisión por venta segura.
Plan premium para tiendas o asociaciones.
Publicidad local.
Convenios con servicios universitarios.
Promoción de productos de segunda mano sostenibles.
Indicadores de negocio futuros
Métrica	Objetivo futuro
Usuarios registrados	Medir adopción
Productos publicados	Medir actividad
Productos vendidos	Medir utilidad real
Mensajes enviados	Medir interacción
Categorías más usadas	Detectar demanda
Usuarios activos mensuales	Medir retención
Conclusión

NebriPop es un proyecto adecuado para demostrar el uso profesional de IA aplicada al desarrollo de software. Permite trabajar con requisitos reales, arquitectura, base de datos, frontend, backend, testing, documentación y despliegue.

El valor principal del proyecto no está solo en crear una aplicación tipo marketplace, sino en demostrar un proceso completo de desarrollo asistido por IA, usando Antigravity como herramienta de alto nivel y OpenCode como herramienta de implementación técnica.

El proyecto será evaluable tanto por su resultado funcional como por la calidad de su documentación, arquitectura, agentes, prompts, metodología, testing y decisiones técnicas.