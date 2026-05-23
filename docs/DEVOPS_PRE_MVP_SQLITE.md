# DevOps: Ejecución de la prueba Pre-MVP SQLite

## Requisitos Previos
- Instalar Rust (`rustup default stable`).
- Instalar target WebAssembly: `rustup target add wasm32-unknown-unknown`.
- Instalar Trunk: `cargo install trunk`.

## Instrucciones de Ejecución

### 1. Iniciar el Backend (Axum + SQLite)
Abre una terminal y ejecuta:
```powershell
cd experiments/pre_mvp_nebripop_sqlite/backend
cargo run
```
El backend se inicializará y autogenerará el archivo `.db` en la carpeta `database`.
Mantenlo corriendo. Estará disponible en `http://127.0.0.1:3000`.

### 2. Iniciar el Frontend (Leptos)
Abre **otra pestaña** de terminal y ejecuta:
```powershell
cd experiments/pre_mvp_nebripop_sqlite/frontend
trunk serve --address 127.0.0.1 --port 8081 --open
```
Esto compilará el frontend a WASM, levantará un servidor local y abrirá la aplicación en `http://127.0.0.1:8081`.

## URLs Importantes
- Backend Health: http://127.0.0.1:3000/health
- Backend API: http://127.0.0.1:3000/products
- Frontend: http://127.0.0.1:8081

## Recomendaciones y Solución de problemas
- Ejecutar backend antes que frontend.
- No cerrar la terminal del backend ni la del frontend.
- Usar puertos 3000 y 8081. Si hay conflicto, cambiar el puerto del frontend en el comando trunk.
- **Persistencia**: La DB reside en `experiments/pre_mvp_nebripop_sqlite/database/nebripop_pre_mvp.db`. Si quieres resetear el estado, simplemente borra ese archivo y reinicia el backend.
