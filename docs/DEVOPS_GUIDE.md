# Guía DevOps - NebriPop

## 1. Entorno Local y Puertos Usados

NebriPop está diseñado para ejecutarse localmente con una configuración cliente-servidor, comunicándose mediante HTTP REST.

- **Frontend (Leptos)**:
  - Puerto: `8081`
  - URL local: `http://127.0.0.1:8081`
  - Herramienta de desarrollo: `trunk serve`
- **Backend (Axum)**:
  - Puerto: `3000`
  - URL local: `http://127.0.0.1:3000`
  - Ejecución: `cargo run`
- **Base de Datos**:
  - Motor: SQLite 3
  - Ruta del archivo: `database/nebripop.db`

## 2. Variables de Entorno y Secretos
- En el entorno local, no es estrictamente necesario un archivo `.env` para el MVP, ya que la base de datos usa rutas relativas o absolutas predefinidas y no hay servicios externos críticos.
- Si se requiere en el futuro, se utilizará un `.env.example` en la raíz. **No se deben subir nunca archivos `.env` al control de versiones.**

## 3. Solución de Errores Comunes (Troubleshooting)

### Error: Puerto en uso (Address already in use)
Si el frontend o el backend no pueden arrancar, es probable que los puertos `8081` o `3000` estén ocupados por procesos anteriores.
- **Solución**: Detén el proceso anterior (Ctrl + C) o busca el PID que ocupa el puerto y elimínalo.

### Error: Trunk no encuentra el target wasm32
- **Causa**: Falta el target `wasm32-unknown-unknown`.
- **Solución**: Ejecuta `rustup target add wasm32-unknown-unknown`.

### Error: Base de datos no encontrada / "no such table"
- **Causa**: El archivo `nebripop.db` no existe, no está en la ruta esperada, o no se han ejecutado las migraciones/esquema.
- **Solución**: Ejecuta `sqlite3 database/nebripop.db < database/schema.sql` desde la raíz del proyecto. Asegúrate de iniciar el backend desde la carpeta `/backend`.

### Error de compilación en Windows (link.exe no encontrado)
- **Causa**: Faltan las C++ Build Tools.
- **Solución**: Instala "Desktop development with C++" a través de Visual Studio Installer.

### Error: Cambios en el código no se reflejan
- **Causa**: A veces Trunk o Cargo cachean compilaciones de forma inesperada o fallan por artefactos residuales.
- **Solución**: Limpia el target y vuelve a compilar:
  ```bash
  cargo clean
  ```
  Y vuelve a ejecutar el backend o frontend según corresponda.

## 4. Detener Servidores y Limpieza
- Para detener tanto el backend como el frontend, simplemente presiona `Ctrl + C` en las respectivas terminales donde se están ejecutando.
- Para limpiar archivos compilados si es necesario: `cargo clean` en la raíz del backend o frontend.
