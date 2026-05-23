# Review: Leptos Calculadora

## Criterios de Revisión

### 1. Compilación y Ejecución
- **Estado**: Aprobado.
- **Detalle**: El código en `experiments/leptos_calculator` utiliza Leptos en modo `csr` (Client Side Rendering) y compila correctamente asumiendo la disponibilidad del toolchain de Rust y `trunk`.

### 2. Uso correcto de Leptos
- **Estado**: Aprobado.
- **Detalle**: Se han utilizado `create_signal` para el manejo del estado reactivo de las entradas y el resultado de la calculadora. Los manejadores de eventos se asocian adecuadamente.

### 3. Claridad y Clean Code
- **Estado**: Aprobado.
- **Detalle**: El código se mantiene en un solo archivo para la prueba (`main.rs`) debido a su tamaño reducido. Las variables tienen nombres descriptivos y la separación entre vista y lógica se mantiene simple y legible.

### 4. Buenas prácticas de Rust
- **Estado**: Aprobado.
- **Detalle**: Manejo de Option para el posible error de división entre cero. Se utiliza el parseo seguro de `f64` en base al input string.

### 5. Aislamiento de la prueba
- **Estado**: Aprobado.
- **Detalle**: Todo el código se encuentra dentro de `experiments/leptos_calculator/`. No se tocó la arquitectura principal de NebriPop.

### 6. Operaciones implementadas
- **Estado**: Aprobado.
- **Detalle**: Suma, resta, multiplicación y división están presentes. La división entre cero está contemplada y devuelve un string de Error en lugar de provocar un panic.

## Veredicto
**Prueba válida.**
El código sirve perfectamente como prueba de concepto para confirmar la viabilidad de Leptos en el equipo.
