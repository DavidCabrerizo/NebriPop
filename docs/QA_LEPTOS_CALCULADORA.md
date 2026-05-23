# QA Checklist: Leptos Calculadora

## Objetivo
Validar el correcto funcionamiento de la aplicación básica de calculadora implementada con Leptos.

## Casos de Prueba

### Funcionalidad Básica
- [ ] **Suma**: Ingresar 5 y 3, presionar "+". Resultado esperado: 8.
- [ ] **Resta**: Ingresar 10 y 4, presionar "-". Resultado esperado: 6.
- [ ] **Multiplicación**: Ingresar 7 y 6, presionar "*". Resultado esperado: 42.
- [ ] **División (válida)**: Ingresar 20 y 5, presionar "/". Resultado esperado: 4.

### Casos Límite y Manejo de Errores
- [ ] **División por cero**: Ingresar 10 y 0, presionar "/". Resultado esperado: Mensaje de error claro (ej. "Error: División por cero").
- [ ] **Entradas vacías o inválidas**: No ingresar ningún número o ingresar texto. Presionar cualquier operación. Resultado esperado: La aplicación no crashea, maneja el estado de forma segura (ignorando la acción o mostrando error de formato).

### Aislamiento de la Prueba
- [ ] Comprobar que los archivos creados están únicamente dentro de `experiments/leptos_calculator/`.
- [ ] Comprobar que no se han modificado archivos críticos del proyecto (`PRD.md`, `backend/`, base de datos, migraciones).
