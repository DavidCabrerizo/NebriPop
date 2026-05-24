# Esquema de Base de Datos (Fase 2 MVP)

Este documento describe las tablas diseñadas por **DatabaseAgent** para el MVP de NebriPop.

## Tablas Principales

### 1. `users`
Almacena la información básica de los usuarios.
- `password_hash`: Se utilizará para autenticación futura (sin contraseñas en texto plano).
- `email`: Debe ser único.

### 2. `categories`
Categorías de productos (tipo Wallapop).
- `name`: Único. Se inicializan mediante el archivo de *seeds*.

### 3. `products`
Información principal de los productos publicados.
- `price`: Validado para que sea `>= 0`.
- `condition`: Enumeración simulada (new, like_new, good, used, damaged).
- `status`: Estado del producto (available, reserved, sold).
- Relaciona a un usuario (vendedor) y una categoría obligatoria.

## Tablas Preparadas (Para fases futuras)

### 4. `product_images`
Permitirá asignar múltiples imágenes a un mismo producto. En la versión más básica se usará `main_image_url` en la tabla de `products`, pero la base queda lista para expandirse.

### 5. `favorites`
Permitirá a los usuarios guardar productos como favoritos. Relación Muchos a Muchos entre `users` y `products`.

### 6. `messages`
Preparada para un sistema básico de contacto (mensajería) entre un potencial comprador y el vendedor sobre un producto específico.

## Índices
Se han creado índices clave para optimizar búsquedas frecuentes:
- Búsquedas de productos por categoría, usuario, estado, precio y ubicación.
- Claves foráneas para imágenes, favoritos y mensajes.
