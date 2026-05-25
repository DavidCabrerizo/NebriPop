# QA Plan y Resultados: Fase 6 — Sistema Básico de Usuarios

## Objetivo
Validar la correcta implementación de registro, login y asociación de productos con el usuario en el backend de Axum y el frontend de Leptos utilizando SQLite.

## Casos de Prueba Backend

1. **Registro Exitoso**
   - **Endpoint:** `POST /auth/register`
   - **Acción:** Enviar un body con name, email y password.
   - **Resultado Esperado:** Código 200, retorna usuario sin `password_hash`.
   - **Estado:** ✅ Validado

2. **Registro - Email Duplicado**
   - **Endpoint:** `POST /auth/register`
   - **Acción:** Enviar el mismo email de la prueba anterior.
   - **Resultado Esperado:** Código 400, "El email ya está registrado".
   - **Estado:** ✅ Validado

3. **Registro - Password Corto**
   - **Endpoint:** `POST /auth/register`
   - **Acción:** Enviar un password con menos de 6 caracteres.
   - **Resultado Esperado:** Código 400.
   - **Estado:** ✅ Validado

4. **Login Exitoso**
   - **Endpoint:** `POST /auth/login`
   - **Acción:** Enviar email y password correcto.
   - **Resultado Esperado:** Código 200, "Login correcto".
   - **Estado:** ✅ Validado

5. **Login Fallido**
   - **Endpoint:** `POST /auth/login`
   - **Acción:** Enviar contraseña incorrecta.
   - **Resultado Esperado:** Código 401, "Email o contraseña incorrectos".
   - **Estado:** ✅ Validado

6. **Perfil de Usuario**
   - **Endpoint:** `GET /users/:id`
   - **Acción:** Consultar el id de un usuario existente.
   - **Resultado Esperado:** Código 200, devuelve perfil.
   - **Estado:** ✅ Validado

7. **Productos del Usuario**
   - **Endpoint:** `GET /users/:id/products`
   - **Acción:** Consultar productos.
   - **Resultado Esperado:** Código 200, devuelve lista vacía o lista de productos.
   - **Estado:** ✅ Validado

## Casos de Prueba Frontend

1. **Navegación Auth**
   - **Acción:** Ir a `/login` y `/register`.
   - **Resultado Esperado:** Formularios visibles, botones correctos.
   - **Estado:** ✅ Validado

2. **Flujo de Registro**
   - **Acción:** Rellenar formulario y enviar.
   - **Resultado Esperado:** Muestra éxito, enlace al login.
   - **Estado:** ✅ Validado

3. **Flujo de Login y LocalStorage**
   - **Acción:** Iniciar sesión y comprobar `localStorage`.
   - **Resultado Esperado:** Redirección al home, `user_id` guardado, "Iniciar sesión" se convierte en "Perfil".
   - **Estado:** ✅ Validado

4. **Publicar Producto**
   - **Acción:** Entrar a `/products/new` sin usuario y luego con usuario.
   - **Resultado Esperado:** Sin usuario: error. Con usuario: usa `user_id` y publica.
   - **Estado:** ✅ Validado

## Verificación Base de Datos
- Tabla `users` no contiene JSON.
- `password_hash` se guarda encriptado mediante `argon2/bcrypt`.
- Las consultas son preparadas y previenen Inyección SQL.
- **Estado:** ✅ Validado

## Veredicto QAAgent
Las funcionalidades del MVP para usuarios básicos están implementadas correctamente. No hay regresiones con las fases anteriores.
