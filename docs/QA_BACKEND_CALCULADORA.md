# QA Checklist: Backend Axum Calculadora

## Objetivo
Validar que el API REST programado en Rust con el framework Axum funcione correctamente, maneje el estado en memoria, reciba y devuelva JSON y realice las validaciones requeridas.

## Casos de Prueba

### Servidor y Estado Base
- [ ] **Healthcheck**: Hacer GET a `http://127.0.0.1:3000/health`. Resultado esperado: `{"status": "ok", "message": "Backend de prueba funcionando"}`.
- [ ] **Historial vacío inicial**: Hacer GET a `http://127.0.0.1:3000/operations`. Resultado esperado: `[]`.

### Guardar Operaciones Correctas (POST /operations)
- [ ] **Suma**: Enviar `{"left_value":10.0,"operator":"+","right_value":5.0,"result":15.0}`. Resultado esperado: JSON con el id autoincrementado (ej. 1) y los datos guardados, estado 200 OK.
- [ ] **Resta**: Enviar `{"left_value":20.0,"operator":"-","right_value":8.0,"result":12.0}`. Resultado esperado: id = 2.
- [ ] **Multiplicación**: Enviar `{"left_value":4.0,"operator":"*","right_value":3.0,"result":12.0}`. Resultado esperado: id = 3.
- [ ] **División (válida)**: Enviar `{"left_value":10.0,"operator":"/","right_value":2.0,"result":5.0}`. Resultado esperado: id = 4.

### Validaciones de Seguridad (POST /operations)
- [ ] **Rechazar división entre cero**: Enviar `{"left_value":10.0,"operator":"/","right_value":0.0,"result":0.0}`. Resultado esperado: 400 Bad Request, con el mensaje "División por cero no permitida".
- [ ] **Rechazar operadores no válidos**: Enviar `{"left_value":5.0,"operator":"^","right_value":2.0,"result":25.0}`. Resultado esperado: 400 Bad Request, con el mensaje de "Operador no válido".
- [ ] **Rechazar resultados incorrectos**: Enviar `{"left_value":10.0,"operator":"+","right_value":5.0,"result":99.0}`. Resultado esperado: 400 Bad Request, con el mensaje indicando que el resultado no es correcto.

### Verificación de Almacenamiento
- [ ] **Historial persistente (en memoria)**: Hacer un GET final a `http://127.0.0.1:3000/operations`. Resultado esperado: Ver un array JSON con las 4 operaciones guardadas exitosamente durante la prueba.

### Aislamiento de la Prueba
- [ ] Verificar que no hay modificaciones en `backend/`, base de datos o configuraciones de NebriPop. Todo reside en `experiments/backend_calculator_test/`.
