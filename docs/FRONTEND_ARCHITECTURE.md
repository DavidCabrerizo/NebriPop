# Arquitectura Frontend - Fase 4

## 1. Visión General
El frontend de NebriPop se construirá utilizando **Rust + Leptos** y se compilará a WebAssembly usando **Trunk**. La arquitectura seguirá un patrón de componentes funcionales y reactivos típico de frameworks modernos (como React/Solid), pero aprovechando el tipado fuerte y el rendimiento de Rust.

## 2. Estructura de Directorios Propuesta

```
frontend/
├── Cargo.toml          # Dependencias y configuración (Leptos, Serde, Reqwest/gloo-net)
├── index.html          # Punto de entrada HTML para Trunk
├── styles.css          # Estilos base
└── src/
    ├── main.rs         # Punto de entrada de la aplicación (montaje en DOM)
    ├── app.rs          # Componente raíz y configuración de Rutas (Leptos Router)
    ├── api/            # Comunicación con el Backend Axum
    │   ├── client.rs   # Configuración base del cliente HTTP
    │   ├── products.rs # Funciones GET /products, GET /products/:id, POST /products
    │   └── categories.rs # Funciones GET /categories
    ├── components/     # Componentes visuales reutilizables
    │   ├── product_card.rs
    │   ├── product_list.rs
    │   ├── product_form.rs
    │   └── error_message.rs
    ├── pages/          # Vistas principales de la aplicación
    │   ├── home.rs
    │   ├── product_detail.rs
    │   └── create_product.rs
    └── models/         # Tipos y structs (Serialización/Deserialización con Serde)
        ├── product.rs
        └── category.rs
```

## 3. Decisiones Arquitectónicas (Fase 4)
- **Framework:** Leptos en modo Client-Side Rendering (CSR). Es la forma más sencilla de empezar un SPA (Single Page Application) e integrarlo con una API externa sin complicar el despliegue inicial con SSR.
- **Cliente HTTP:** Se usará `reqwest` o `gloo-net` (reqwest es muy común en Rust, habilitando la feature `wasm-bindgen`).
- **Gestión de Estado:** El estado asíncrono (datos del backend) se manejará mediante los `Resource` nativos de Leptos (`create_resource`), que interactúan perfectamente con `<Suspense>` y `<ErrorBoundary>`.
- **Enrutamiento:** `leptos_router` para navegar entre la Home, el detalle de producto y el formulario de creación.
- **Estilos:** Se utilizará un archivo `styles.css` tradicional importado desde `index.html` para mantener las dependencias bajas y el MVP simple, evitando la sobreingeniería con frameworks CSS hasta que se justifique.

## 4. Modelado de Datos Compartidos
El frontend duplicará temporalmente (o importará si fuera un monorepo real estructurado como workspace) los structs definidos en el backend (e.g., `Product`, `Category`). Se recomienda usar `serde::Deserialize` para recibir los datos de Axum.

## 5. Prevención de Sobreingeniería
En esta fase, **no** se implementará:
- Gestión de estado global compleja (Context API para usuarios se dejará para la fase de Login).
- Internacionalización (i18n).
- Server-Side Rendering (SSR).
- Frameworks CSS pesados (salvo CSS simple).
