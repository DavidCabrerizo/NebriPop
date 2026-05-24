# Arquitectura del Backend de NebriPop

## Decisiones Técnicas (RustArchitectAgent)

En esta Fase 3, se implementará el backend base de NebriPop utilizando **Rust** y el framework web **Axum**. Para el acceso a la base de datos **SQLite**, se utilizará **SQLx**, ya que permite consultas asíncronas seguras, en tiempo de compilación y es el estándar de facto para aplicaciones web robustas en Rust.

### Estructura de Carpetas

La arquitectura seguirá un patrón de capas para separar responsabilidades (Clean Architecture simplificada):

```text
backend/
├── Cargo.toml
├── src/
│   ├── main.rs          # Punto de entrada, configuración del servidor, CORS y pool de conexiones.
│   ├── config.rs        # Lectura de variables de entorno (DATABASE_URL, PORT).
│   ├── db.rs            # Configuración de SQLx y pool de base de datos.
│   ├── errors.rs        # Definición de errores personalizados y conversión a respuestas JSON.
│   ├── models.rs        # Estructuras de datos que representan las tablas en SQLite.
│   ├── dto/             # Data Transfer Objects (Payloads de entrada y salida, ej. CreateProductDto).
│   ├── routes/          # Definición de los routers de Axum (health, categories, products).
│   ├── handlers/        # Lógica de los endpoints (recepción de request, validación, respuesta HTTP).
│   ├── services/        # (Opcional) Lógica de negocio (orquestación entre handler y repositorios).
│   └── repositories/    # Lógica de acceso a datos (ejecución de sentencias SQL).
└── tests/               # Tests de integración.
```

### Separación de Responsabilidades

1. **Routes**: Exclusivamente encadenar rutas con sus handlers correspondientes.
2. **Handlers**: Validar la petición HTTP, extraer parámetros o body, llamar a services/repos, y devolver una respuesta HTTP estructurada (JSON o Error HTTP).
3. **Services**: Lógica de negocio. En esta fase temprana, los handlers pueden llamar directamente a los repositorios si no hay lógica compleja, para evitar sobreingeniería.
4. **Repositories**: Centraliza todas las consultas SQL. Aquí se usa SQLx para leer o escribir en la base de datos SQLite de forma segura usando statements parametrizados.
5. **Models vs DTOs**: 
   - `Models` se usan para la representación interna y lo que devuelve el Repositorio.
   - `DTOs` (Data Transfer Objects) definen los datos esperados de entrada del cliente (ej. `CreateProductDto`) y salida, aislando a los clientes de los cambios en la base de datos.

### Recomendaciones para RustBackend

- **Asincronía**: Utilizar tokio y aprovechar el soporte asíncrono de Axum y SQLx.
- **Seguridad en BD**: Las consultas deben estar parametrizadas para evitar SQL Injection (SQLx lo hace por defecto con la macro `query!`).
- **Manejo de Errores**: Usar `thiserror` para modelar errores internos y convertirlos explícitamente a respuestas JSON usando el trait `IntoResponse` de Axum. Nunca filtrar detalles sensibles de base de datos en los errores 500.
- **Validaciones**: Las validaciones básicas (ej. precio > 0, título no vacío) deben hacerse en el momento de instanciar el DTO o en el handler, antes de llegar a la base de datos.
- **CORS**: Configurar `tower-http` en el router principal para permitir peticiones desde `http://127.0.0.1:8081` (el futuro frontend).
