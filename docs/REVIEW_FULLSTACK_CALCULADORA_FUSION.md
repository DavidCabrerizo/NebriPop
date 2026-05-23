# Review: Fullstack Calculadora Fusión

## Criterios de Revisión

### 1. Separación correcta entre backend y frontend
- **Estado**: Aprobado.
- **Detalle**: El proyecto se ha separado en dos crates independientes (`backend` y `frontend`) dentro de la carpeta `fullstack_calculator_fusion`, facilitando su compilación paralela y su gestión de dependencias.

### 2. Uso correcto de Frameworks (Axum & Leptos)
- **Estado**: Aprobado.
- **Detalle**: Backend implementa las rutas de Axum con `CorsLayer` (desde `tower-http`) para permitir llamadas del frontend en el puerto 8081. Frontend usa Leptos con `create_signal`, `create_resource` y `Suspense` para la carga asíncrona del historial, gestionando efectos secundarios de forma idiomática mediante `spawn_local` y `reqwest`.

### 3. Comunicación mediante API REST
- **Estado**: Aprobado.
- **Detalle**: El frontend realiza serialización JSON (`serde`) usando `reqwest` a las URLs locales del backend.

### 4. Buenas prácticas de Rust
- **Estado**: Aprobado.
- **Detalle**: No se ha usado SQLite. Manejo concurrente de estado protegido. No se observan bloqueos innecesarios en el UI, y los errores de red devuelven strings que se inyectan reactivamente en el DOM de Leptos.

## Veredicto
**Prueba válida.**
La prueba cumple con éxito la demostración de la interoperabilidad del stack seleccionado (Rust + Leptos + Axum) validando de extremo a extremo las comunicaciones HTTP, serialización y estado reactivo sin romper ninguna regla del repositorio.
