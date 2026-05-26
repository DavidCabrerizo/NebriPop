# UI/UX: Fase 7 - Búsqueda y Filtros de Productos

## Objetivo
Permitir a los usuarios encontrar productos de forma rápida y sencilla en NebriPop, un marketplace de segunda mano, mediante herramientas de búsqueda y filtrado intuitivas.

## Flujos de Navegación
1. **Acceso Inicial**: El usuario entra a la página de inicio (Home) y ve una barra de búsqueda destacada en la parte superior del listado de productos, con un panel lateral (o sección colapsable) para los filtros más detallados.
2. **Búsqueda por Texto**: El usuario escribe en la barra de búsqueda y presiona enter o hace clic en "Buscar". El listado de productos se actualiza inmediatamente.
3. **Aplicación de Filtros**: El usuario selecciona opciones en el panel de filtros (categoría, precio, estado, etc.). Los filtros se aplican al enviar el formulario de filtros.
4. **Sin Resultados**: Si la combinación de búsqueda y filtros no devuelve resultados, se muestra un "Empty State" claro invitando a cambiar los criterios de búsqueda.

## Componentes y Disposición Visual

### 1. Barra de Búsqueda (`SearchBar`)
- **Ubicación**: En la parte superior de la página principal o listado.
- **Diseño**: Un input de texto amplio con un icono de lupa y un placeholder descriptivo (ej. "Buscar productos, marcas, modelos...").
- **Acción**: Botón "Buscar" adjunto al input.

### 2. Panel de Filtros (`ProductFilter`)
- **Ubicación**: Lateral izquierdo en escritorio. En móvil, puede ser un botón "Filtros" que abra un modal o sección colapsable.
- **Controles**:
  - **Categoría**: `select` dropdown con las categorías disponibles.
  - **Precio**: Dos `input type="number"` para precio Mínimo y Máximo (en euros).
  - **Condición**: `select` dropdown (Nuevo, Como Nuevo, Bueno, Usado, Dañado).
  - **Ubicación**: Input de texto corto para indicar la ciudad o zona.
  - **Estado**: `select` dropdown (Disponible, Reservado, Vendido) - Opcional para compradores, útil para ver qué hay disponible.
  - **Orden**: Dropdown de ordenación (Más recientes, Precio: Menor a Mayor, Precio: Mayor a Menor).
- **Acciones**:
  - Botón principal: "Aplicar Filtros".
  - Botón secundario: "Limpiar" (para resetear todos los campos y volver a mostrar todos los productos).

### 3. Listado de Productos (`ProductList`)
- **Diseño**: Cuadrícula (Grid) responsive de tarjetas de producto (igual que en fases anteriores).
- **Feedback**:
  - Mensaje "Buscando..." o esqueleto de carga mientras se obtienen los resultados.
  - Mensaje tipo "No se han encontrado productos con estos filtros" cuando la lista esté vacía.

## Usabilidad y Responsive
- **Escritorio**: Filtros visibles en la columna izquierda, listado en la columna derecha.
- **Móvil / Tablet**: Barra de búsqueda arriba. Botón "Filtros" que expande las opciones. Tarjetas de producto adaptadas al ancho de la pantalla (1 o 2 columnas).
- **Validación Visual**: Si el usuario introduce precio mínimo mayor al máximo, el formulario no debe permitir aplicar o debe mostrar un texto en rojo indicando el error.

## Accesibilidad
- Los inputs deben tener `<label>` visibles o `aria-label` descriptivos.
- Contraste adecuado en el texto de los "Empty States".
- Navegación mediante teclado (Tabulación lógica: Búsqueda -> Filtros -> Listado).
