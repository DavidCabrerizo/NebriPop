# Test de Fusión Full-Stack: Calculadora Leptos + Axum

## Objetivo de la prueba
Comprobar la integración del stack completo de Rust seleccionado para el proyecto NebriPop, unificando un cliente Leptos (Frontend interactivo) con una API Axum (Backend).

## Contexto
Anteriormente se habían validado de forma aislada las capacidades de Axum (para guardar operaciones) y Leptos (para realizar cálculos en el navegador). Esta prueba fusiona ambos flujos en un entorno aislado (`experiments/fullstack_calculator_fusion`) para verificar las comunicaciones de red y el CORS en Rust.

## Arquitectura y Comunicación
- **Backend (Axum)**: Expone el puerto `3000`. Habilita capa CORS (`tower_http::cors::CorsLayer`) para permitir peticiones POST y GET. 
- **Frontend (Leptos)**: Utiliza `reqwest` en `wasm32` para interactuar con la API de Axum. Utiliza el primitivo `create_resource` para realizar peticiones GET y re-pintar automáticamente la lista de operaciones mediante un componente `<Suspense>`.
- **Intercambio**: JSON serializado y tipado compartiendo estructuras similares (`Operation`, `NewOperation`).

## Agentes involucrados
- **Agent Manager**: Dirección general del test.
- **RustArchitectAgent**: Planificación de la estructura en dos crates separados (`backend` y `frontend`).
- **RustBackend**: Adición del sistema de CORS y readaptación de la API.
- **RustFrontend**: Implementación de llamadas HTTP (con `reqwest`), componentes asíncronos y gestión de errores de red.
- **QAAgent**: Validación de interacciones extremo a extremo.
- **ReviewerAgent**: Validación de arquitectura de red y estado reactivo.
- **DocumentationAgent**: Elaboración del presente informe.

## Cómo ejecutar

1. **Backend**:
   ```bash
   cd experiments/fullstack_calculator_fusion/backend
   cargo run
   ```
2. **Frontend** (en otra terminal):
   ```bash
   cd experiments/fullstack_calculator_fusion/frontend
   trunk serve --address 127.0.0.1 --port 8081 --open
   ```

## Resultados
Al pulsar un botón matemático en el frontend, se genera el cálculo y se dispara de forma asíncrona una llamada a la API (POST `/operations`). Una vez devuelto el código de éxito, Leptos emite un `.refetch()` al resource que se encarga del GET a `/operations`, ocasionando que la lista inferior de la pantalla se actualice de inmediato en tiempo real. 
En caso de división por cero o servidor inalcanzable, la interfaz muestra errores visuales de forma declarativa.

## Corrección de integración frontend-backend
Tras una primera validación manual del backend con PowerShell, se detectó que el frontend original no persistía automáticamente las operaciones en el servidor. 
Se corrigió la integración en Leptos para garantizar que:
- Cada vez que se ejecuta una operación matemática válida, se envía un `POST /operations` al backend Axum con el payload exacto.
- Al recibir un código de éxito (200 OK), el frontend ejecuta explícitamente un `.refetch()` que dispara un `GET /operations` para actualizar el historial mostrado en pantalla.
- Se ha añadido logging explícito en el frontend (`leptos::logging::log!`) para facilitar la depuración.
- Los datos se siguen guardando temporalmente en la memoria del backend, por lo que si el servidor Axum se reinicia, el historial se borra.

## Conclusiones
La fusión confirma que el stack de Rust en ambos extremos provee un tipado seguro en ambas direcciones, agilidad en la serialización y una reactividad excelente mediante Leptos en conjunto con una respuesta en milisegundos por parte de Axum. El stack base es un rotundo éxito.

---

### Resumen preparado para registrar en docs/Logs.md

- **Fecha:** 22 de mayo de 2026.
- **Agente responsable:** Agent Manager.
- **Acción realizada:** Fusión e integración técnica full-stack de Leptos y Axum mediante llamadas HTTP y CORS.
- **Archivos creados:**
  - `docs/Prompts/PruebaFullStackCalculadoraFusionV1.txt`
  - `docs/FULLSTACK_CALCULADORA_FUSION_TEST.md`
  - `docs/QA_FULLSTACK_CALCULADORA_FUSION.md`
  - `docs/REVIEW_FULLSTACK_CALCULADORA_FUSION.md`
  - Proyecto dual en `experiments/fullstack_calculator_fusion/` (frontend y backend).
- **Motivo del cambio:** Asegurar que Leptos y Axum se comunican de forma efectiva antes de iniciar la arquitectura oficial.
- **Funcionalidades implementadas:** Comunicación REST real en Rust (WASMBindgen + reqwest), carga asíncrona reactiva con `<Suspense>` y persistencia en estado temporal en el servidor Axum.
- **Próximo paso recomendado:** Dar por cerrados los experimentos tecnológicos y comenzar a estructurar el proyecto base de NebriPop (Fase 1).
