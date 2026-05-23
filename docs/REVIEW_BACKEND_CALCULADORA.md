# Review: Backend Axum Calculadora

## Criterios de Revisión

### 1. Uso correcto de Axum
- **Estado**: Aprobado.
- **Detalle**: El backend inicializa `tokio`, crea un `Router` y bindea los puertos usando el módulo de HTTP y los extractores de Axum (`Json`, `State`). El estado compartido se maneja en memoria.

### 2. Código simple y mantenible
- **Estado**: Aprobado.
- **Detalle**: El código se encapsula limpiamente en `main.rs`, dada la pequeña magnitud de la prueba. Los handlers y las estructuras modelo (Operation, NewOperation) son muy legibles y concisas.

### 3. Buenas prácticas de Rust
- **Estado**: Aprobado.
- **Detalle**: Manejo concurrente de memoria con `Arc<Mutex<AppState>>` previene las condiciones de carrera en un entorno asíncrono. En lugar de hacer un `unwrap()` ciego, los errores devolviendo `(StatusCode, Json<ErrorResponse>)` siguen la idiomática de Axum.

### 4. Uso correcto de JSON
- **Estado**: Aprobado.
- **Detalle**: `serde` y `serde_json` se utilizan apropiadamente con derives de `Serialize` y `Deserialize` para transformar de manera transparente los inputs y outputs de las peticiones REST.

### 5. Validaciones y Manejo de errores
- **Estado**: Aprobado.
- **Detalle**: La lógica evalúa (1) el operador, (2) la posible división por cero, y (3) que el `result` coincida numéricamente con el resultado esperado utilizando tolerancias para coma flotante. Los retornos devuelven el status `BAD_REQUEST`.

### 6. Cumplimiento de restricciones
- **Estado**: Aprobado.
- **Detalle**: No se ha utilizado SQLite. No se implementa lógica funcional de NebriPop. Todo se almacena temporalmente y queda guardado en la carpeta aislada `experiments/backend_calculator_test`.

## Veredicto
**Prueba válida.**
El backend demuestra solidez al levantar un servidor, manejar rutas asíncronamente con serialización JSON, y coordinar un estado mutado concurrentemente en memoria.
