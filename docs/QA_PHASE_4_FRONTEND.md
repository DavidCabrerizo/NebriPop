# QA Checklist - Fase 4 (Frontend)

## Requisitos Previos
- [ ] El backend de Axum compila y se ejecuta en `http://127.0.0.1:3000`.
- [ ] La base de datos tiene datos semilla (productos y categorías creados en Fase 2/3).
- [ ] El frontend compila con `trunk build` sin errores.
- [ ] Trunk levanta el servidor frontend en `http://127.0.0.1:8081`.

## Pruebas de Integración y Visuales
- [ ] **Home (Listado)**: Se solicitan productos a `GET /products` y se muestran.
- [ ] **Home (Carga)**: Se muestra mensaje de carga antes de renderizar.
- [ ] **Home (Error)**: Si el backend está apagado, se muestra un mensaje de error claro en pantalla.
- [ ] **Detalle de Producto**: Al hacer clic en un producto, navega a la URL correcta y carga `GET /products/:id`.
- [ ] **Detalle (Error 404)**: Si se fuerza una URL con un ID inexistente, se maneja el error.
- [ ] **Formulario (Categorías)**: Al entrar a publicar producto, el select carga las categorías de `GET /categories`.
- [ ] **Formulario (Validaciones)**: Si se deja el título o descripción vacío, muestra error en el frontend y no envía la petición.
- [ ] **Formulario (Creación)**: Al enviar un formulario válido, `POST /products` retorna éxito.
- [ ] **Formulario (Redirección/Mensaje)**: Tras crear un producto, redirige al Home o muestra mensaje de éxito y la lista se actualiza.

## Restricciones
- [ ] No hay código funcional de login implementado.
- [ ] El `user_id` enviado en el formulario es temporal (1) y está documentado.
