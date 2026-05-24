# Phase 5: Integración Frontend + Backend + SQLite

Este documento establece las directrices arquitectónicas y detalla el cierre de la Fase 5 de NebriPop, validando la integración completa y coherente entre Leptos, Axum y SQLite.

## 1. Objetivo de la fase
Validar la integración completa del stack tecnológico (Leptos + Axum + SQLite), asegurando que el frontend y el backend se comunican correctamente y que los datos se persisten de forma real y segura en la base de datos.

## 2. Flujo validado
Se verificó el flujo completo bidireccional: el frontend en Leptos realiza peticiones HTTP (mediante `reqwest`), el backend en Axum recibe y procesa la solicitud manejando los CORS de forma adecuada, interactúa de forma asíncrona con la base de datos vía `sqlx` y retorna las respuestas para que Leptos las renderice de forma reactiva.

## 3. Base de datos SQLite utilizada
Se validó la conexión al esquema de base de datos relacional diseñado en la Fase 2, gestionando las transacciones de manera eficiente mediante un pool de conexiones (`&SqlitePool`) inyectado en el `State` de Axum.

## 4. Generación de database/nebripop.db
Se confirmó la correcta inicialización y generación automática del archivo de base de datos local `nebripop.db` mediante el motor `sqlx`, ejecutando las migraciones (`schema.sql`) y la siembra de datos (`seeds.sql`) necesarias para la app.

## 5. Endpoints validados
Se ha comprobado exitosamente la operatividad de los endpoints fundamentales del MVP (ej. `/products`), validando métodos `GET` y `POST`, su enrutamiento en Axum, su conversión a JSON y la estructura de manejo de errores (`AppError`).

## 6. Pantallas conectadas
Las vistas desarrolladas previamente en Leptos (como el listado general de productos, la vista en detalle y el formulario de creación) han sido enlazadas satisfactoriamente al API, abandonando su estado aislado.

## 7. Persistencia real
Se ha verificado empíricamente que cualquier creación u operación de estado gestionada en el frontend impacta de forma inmediata en los registros físicos de SQLite, respetando las validaciones e integridad referencial dictadas en el esquema (ej. validaciones de precios).

## 8. Eliminación de uso de JSON como persistencia
Se ha validado y confirmado la eliminación absoluta de mocks, objetos en memoria de prueba y lectura/escritura de archivos `.json` locales, consolidando a la base de datos SQLite como única fuente de verdad real.

## 9. Pruebas realizadas
- Simulación de tráfico de datos (GET/POST) entre frontend y backend.
- Verificación en componentes Leptos de estados de red como cargas (`<Suspense>`) y manejo de errores (`ErrorBoundary`).
- Pruebas de compilación correctas sin errores (resolución de previos errores de borrowing en frontend).

## 10. Riesgos detectados
- **Variables de Entorno y Rutas:** El uso de rutas relativas o variables hardcodeadas como `http://127.0.0.1:3000` y `sqlite://../database/nebripop.db`. Mitigación: Forzar el uso de `.env` unificado.
- **CORS:** El uso de `allow_headers(Any)` puede bloquear preflights bajo configuraciones futuras; será necesario ajustar a headers explícitos.
- **Errores no procesados UI:** El frontend no debe devolver errores en crudo al usuario; requiere parseo de la respuesta HTTP a mensajes amigables.

## 11. Resultado final
Integración del stack técnico full-stack (Leptos + Axum + SQLite) superada con un resultado estable. Las bases fundamentales de la aplicación logran interactuar de manera fluida bajo el modelo de cliente-servidor con estado persistente. La Fase 5 se considera cerrada y validada.

## 12. Próxima fase recomendada
Configurar e invocar a GitHubAgent para realizar commit y push del cierre de la Fase 5. Una vez respaldado, avanzar a la próxima fase centrada en la implementación de la autenticación/registro de usuarios o el cierre final de funcionalidades core del MVP.
