# Guía de Instalación y Ejecución Local - NebriPop

## 1. Requisitos Previos
Para ejecutar el proyecto NebriPop en tu máquina local, necesitas tener instalado lo siguiente:

- **Rust y Cargo**: Lenguaje y gestor de paquetes. [Instalar Rust](https://rustup.rs/)
- **Trunk**: Herramienta de compilación para frontend WebAssembly (Leptos).
  ```bash
  cargo install trunk
  ```
- **Target wasm32-unknown-unknown**: Requerido por Trunk para compilar a WebAssembly.
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **SQLite3**: Motor de base de datos.
- **Visual Studio Build Tools (C++ build tools)**: Necesario en Windows para compilar dependencias de Rust (incluido si instalas Rust via `rustup-init.exe` eligiendo la opción por defecto en Windows).

## 2. Preparación de la Base de Datos
El proyecto utiliza SQLite como motor de base de datos.

1. Navega a la raíz del proyecto.
2. Asegúrate de que existe el directorio `database/`.
3. Crea la base de datos inicializando el esquema:
   ```bash
   sqlite3 database/nebripop.db < database/schema.sql
   ```
4. (Opcional) Puedes añadir datos de prueba (seeds) ejecutando:
   ```bash
   sqlite3 database/nebripop.db < database/seeds.sql
   ```
5. Comprueba que se han creado las tablas correctamente (opcional):
   ```bash
   sqlite3 database/nebripop.db ".tables"
   ```

## 3. Ejecución del Proyecto
El proyecto consta de dos partes: Backend (Axum) y Frontend (Leptos). Deben ejecutarse en terminales separadas.

### Iniciar el Backend (Axum)
En una nueva terminal, navega a la carpeta del backend y ejecuta el servidor:
```bash
cd backend
cargo run
```
El servidor backend se iniciará en `http://127.0.0.1:3000`.

### Iniciar el Frontend (Leptos)
En otra terminal, navega a la carpeta del frontend y ejecuta Trunk:
```bash
cd frontend
trunk serve --address 127.0.0.1 --port 8081 --open
```
Esto abrirá automáticamente la aplicación en tu navegador en `http://127.0.0.1:8081`.

## 4. Comprobación Rápida
Puedes comprobar que el backend funciona correctamente abriendo en tu navegador:
- **Salud del backend**: `http://127.0.0.1:3000/health`
- **Listado de categorías**: `http://127.0.0.1:3000/categories`
- **Listado de productos**: `http://127.0.0.1:3000/products`
