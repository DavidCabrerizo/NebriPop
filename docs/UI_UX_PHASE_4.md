# UX/UI - Fase 4: Frontend Base MVP

## 1. Concepto General
NebriPop será una aplicación tipo Wallapop. El diseño de esta fase se centra en lo fundamental, asegurando una experiencia sencilla y directa para el MVP.

**Colores y Tipografía Recomendados (CSS Simple):**
- **Fondo principal:** Gris muy claro o blanco (`#f9fafb`, `#ffffff`) para mantener la limpieza.
- **Color primario:** Un tono azul o verde vibrante para botones de acción principal (e.g., `#2563eb` - azul).
- **Texto:** Gris oscuro (`#1f2937`) para legibilidad.
- **Tipografía:** Sans-serif estándar (`Inter`, `Roboto`, o system-ui).

## 2. Pantallas Principales

### 2.1. Home / Listado de Productos
- **Header:** Título "NebriPop" a la izquierda. Botón "+ Publicar Producto" a la derecha.
- **Subtítulo:** "Marketplace de productos de segunda mano".
- **Contenido:** Cuadrícula (Grid) de tarjetas de productos.
- **Estados:**
  - *Cargando:* Texto o spinner "Cargando productos...".
  - *Vacío:* "No hay productos disponibles por el momento."
  - *Error:* "Error al cargar los productos."

### 2.2. Tarjeta de Producto (Componente)
- **Diseño:** Tarjeta con bordes suaves, sombra ligera (`box-shadow`), y efecto hover sutil.
- **Contenido:**
  - Imagen (placeholder gris si no hay `main_image_url`).
  - Título (negrita, tamaño medio).
  - Precio (destacado, color primario o texto grande).
  - Ubicación y Estado (texto más pequeño, en gris).
- **Interacción:** Al hacer clic en cualquier parte de la tarjeta, lleva a la vista de Detalle.

### 2.3. Detalle de Producto
- **Navegación:** Botón o enlace de "← Volver al listado" en la parte superior.
- **Contenido Principal:**
  - Imagen destacada en la parte superior.
  - Título y Precio a gran tamaño.
  - Bloque de detalles: Categoría, Estado, Ubicación, Fecha de publicación.
  - Descripción completa del producto.

### 2.4. Formulario para Publicar Producto
- **Diseño:** Centrado en la pantalla o en un contenedor con ancho máximo limitado (`max-w-lg`).
- **Campos:**
  - Título (Input text)
  - Descripción (Textarea)
  - Precio (Input number, con step 0.01)
  - Categoría (Select dropdown, cargado dinámicamente)
  - Estado (Select: Nuevo, Como nuevo, Bueno, Usado, Dañado)
  - Ubicación (Input text)
  - URL de Imagen (Input text, opcional)
- **Botón:** "Publicar Producto" (ancho completo o destacado).
- **Feedback:** Mensajes de error en rojo debajo de los campos si hay validación fallida. Mensaje de éxito en verde al publicar.

## 3. Consideraciones de Usabilidad
- Mensajes claros al usuario si el servidor no responde.
- Formularios con etiquetas explícitas y placeholders de ejemplo.
- Prevención de doble envío: deshabilitar el botón de "Publicar" mientras se envía la petición.
