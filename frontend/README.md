# NebriPop Frontend

Este es el módulo de la interfaz de usuario web para **NebriPop**. Es una SPA (Single Page Application) compilada a WebAssembly usando Rust.

## 🚀 Tecnologías
- **Framework Web:** [Leptos](https://leptos.dev/) (Client-Side Rendering)
- **Comunicación HTTP:** `reqwest`
- **Gestión de Estado:** Señales (*Signals*) finamente reactivas.
- **Estilos:** CSS estándar con enfoque modular y diseño Responsive.
- **Empaquetado:** [Trunk](https://trunkrs.dev/)

## 📌 Estado Actual
**MVP Finalizado (Fase 10).** El frontend implementa:
- Diseño "Wallapop-like" enfocado en UX moderna y fluida.
- Formularios interactivos con carga de imágenes y previsualización de galería.
- Búsqueda en tiempo real sin recarga de página usando URLs como estado de sincronización de filtros.
- Sistema de enrutamiento del lado del cliente.
- Contextos globales para el estado de inicio de sesión.

## ⚙️ Uso
Asegúrate de estar en el directorio `frontend/` y de tener Trunk instalado (`cargo install trunk`). El backend debe estar corriendo en `localhost:3000` para que la aplicación funcione.

```bash
# Iniciar servidor de desarrollo en http://127.0.0.1:8080
trunk serve --open
```
