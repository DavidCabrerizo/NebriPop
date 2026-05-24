# Configuración del Entorno de Desarrollo (NebriPop)

## Requisitos Previos

- **Rust**: Instalar a través de `rustup` (https://rustup.rs/).
- **Cargo**: Incluido con Rust.
- **Trunk**: Herramienta de compilación para frontend Rust (`cargo install trunk`).
- **Target wasm32**: Necesario para el frontend. `rustup target add wasm32-unknown-unknown`.
- **Visual Studio Build Tools**: Necesario en Windows para compilar dependencias C++ (C++ build tools).

## Puertos Previstos
- **Backend**: `3000`
- **Frontend**: `8081`

## Comandos futuros esperados (Ejemplo)
- Backend: `cargo run` (dentro de `backend/`)
- Frontend: `trunk serve` (dentro de `frontend/`)
