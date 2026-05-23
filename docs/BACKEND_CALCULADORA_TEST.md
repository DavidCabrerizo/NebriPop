# Test de Axum: Calculadora Backend API

## Objetivo de la prueba
Validar que Axum es un framework backend eficaz para servir una API REST en Rust antes de aplicarlo al proyecto principal de NebriPop. La prueba consiste en montar una API capaz de validar y guardar un historial de operaciones matemáticas en memoria.

## Motivo de probar Axum
Axum, desarrollado por el equipo de Tokio, es uno de los frameworks más sólidos y ergonómicos de Rust para crear APIs REST. Cuenta con excelente rendimiento, compatibilidad nativa con flujos asíncronos (`tokio`) y fuerte tipado mediante extractores de peticiones (`Json`, `Path`, `State`).

## Agentes que han intervenido
- **Agent Manager**: Dirección general del test y verificación de restricciones.
- **RustArchitectAgent**: Planificación simple, optando por mantener el código en `src/main.rs` dada la pequeña envergadura del experimento.
- **RustBackend**: Desarrollo de la API usando `axum` y `serde` aplicando Clean Code y las convenciones de la prueba.
- **QAAgent**: Redacción de los casos de uso para validar la consistencia matemática y de estado.
- **ReviewerAgent**: Validación de la seguridad del estado concurrente (Arc, Mutex).
- **DocumentationAgent**: Elaboración de este informe.

## Endpoints Disponibles
El servidor escucha por defecto en `http://127.0.0.1:3000`.

- `GET /health`: Comprueba que el servidor está online.
- `GET /operations`: Recupera el historial guardado temporalmente.
- `POST /operations`: Guarda una operación matemática en JSON (si es válida).

## Ejemplos de uso con PowerShell
### 1. Healthcheck
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/health -Method GET
```
### 2. Guardar Operación
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/operations `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"left_value":10.0,"operator":"+","right_value":5.0,"result":15.0}'
```
### 3. Historial de Operaciones
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/operations -Method GET
```

## Almacenamiento
Dado que no se utiliza SQLite por petición explícita, las operaciones se guardan en una estructura compartida protegida por un cerrojo: `Arc<Mutex<AppState>>`. Esto garantiza la seguridad de hilos.

## Conclusiones
Axum se desenvuelve de manera espectacular. La definición de rutas es declarativa y las macros de `serde` evitan tener que escribir lógica manual de parseo. El test es un éxito y demuestra que Rust (con Axum) es apto para construir la API real de NebriPop.

---

### Resumen preparado para registrar en docs/Logs.md
*(A la espera de autorización del Agent Manager)*

- **Fecha:** 22 de mayo de 2026.
- **Agente responsable:** Agent Manager.
- **Acción realizada:** Creación de prueba técnica aislada (Backend Calculadora) usando Rust y el framework Axum.
- **Archivos creados:**
  - `docs/Prompts/PruebaBackendCalculadoraV1.txt`
  - `docs/BACKEND_CALCULADORA_TEST.md`
  - `docs/QA_BACKEND_CALCULADORA.md`
  - `docs/REVIEW_BACKEND_CALCULADORA.md`
  - Código en `experiments/backend_calculator_test/` (Cargo.toml, src/main.rs).
- **Motivo del cambio:** Validar las capacidades de Axum para la implementación de endpoints REST y el uso de un estado concurrente en memoria antes de comenzar con la API de NebriPop.
- **Funcionalidades implementadas:** API REST con endpoints `/health` y `/operations` (GET/POST) validando entradas JSON y persistiendo la información de forma temporal en un Mutex asíncrono.
- **Próximo paso recomendado:** Actualizar `docs/Logs.md` e iniciar la planificación de la arquitectura del proyecto NebriPop (Fase 1).
