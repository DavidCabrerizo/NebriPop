# QA Checklist: Fullstack Calculadora Fusión

## Objetivo
Validar que la integración entre Leptos (Frontend) y Axum (Backend) funciona correctamente mediante llamadas HTTP y CORS, actualizando el estado de la aplicación en el navegador.

## Casos de Prueba

### Servidores Base
- [ ] **Backend**: Arrancar con `cargo run` y validar GET a `http://127.0.0.1:3000/health`.
- [ ] **Frontend**: Arrancar con `trunk serve --address 127.0.0.1 --port 8081` y validar que carga la UI.

### Flujo Completo
- [ ] **Suma y guardado**: Ingresar 10 y 20, presionar `+`. Resultado esperado: Muestra 30.0. Aparece en el historial de operaciones abajo al instante.
- [ ] **Resta y guardado**: Ingresar 15 y 5, presionar `-`. Resultado esperado: Muestra 10.0. Aparece en el historial.
- [ ] **Multiplicación y guardado**: Ingresar 4 y 3, presionar `*`. Resultado esperado: Muestra 12.0. Aparece en el historial.
- [ ] **División válida y guardado**: Ingresar 20 y 4, presionar `/`. Resultado esperado: Muestra 5.0. Aparece en el historial.

### Errores y Validaciones
- [ ] **División por cero**: Ingresar 10 y 0, presionar `/`. Resultado esperado: Muestra "Error: División por cero" y NO envía la petición al servidor (no aparece en el historial).
- [ ] **Servidor Caído**: Detener el servidor Axum, ingresar datos en Leptos y calcular. Resultado esperado: Se muestra un mensaje en rojo indicando "Error de conexión con el backend".

### Aislamiento
- [ ] Validar que todo el código pertenece a `experiments/fullstack_calculator_fusion/`.
