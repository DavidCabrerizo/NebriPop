# Prueba de Flujo de Agentes - PruebaFlujoV1

## 1. Objetivo de la prueba
Validar que todos los agentes técnicos de NebriPop comprenden su rol, respetan la jerarquía impuesta por el Agent Manager, colaboran sin solapamientos, documentan sus pasos adecuadamente y comprenden la regla estricta de no modificar código fuente real ni saltarse fases de desarrollo sin autorización expresa.

## 2. Funcionalidad simulada
**“Publicar un producto en NebriPop”**. Un estudiante registrado debe poder crear un anuncio en la plataforma para vender o intercambiar un artículo académico.

## 3. Agentes participantes
- Agent Manager / Product Manager Agent
- UXDesigner
- RustArchitectAgent
- DatabaseAgent
- RustBackend
- RustFrontend
- QAAgent
- SequrityAgent
- DevopsAgent
- DocumentationAgent
- ReviewerAgent

## 4. Flujo de intervención
El flujo se organiza en cascada: el Agent Manager define las pautas, y de manera secuencial intervienen UX (diseño), Arquitectura (organización de código), Bases de Datos (modelado), Backend y Frontend (implementación del servicio), QA y Seguridad (validaciones y riesgos), DevOps (infraestructura), Documentación (registro de avances) y por último, ReviewerAgent (validación cruzada del cumplimiento de las normas).

## 5. Resultado de cada agente

### Agent Manager
- **Objetivo de la prueba**: Simular la publicación de productos sin ejecutar código real.
- **Funcionalidad simulada**: Formulario para crear producto y persistencia del mismo.
- **Archivos permitidos**: `docs/AGENT_FLOW_TEST.md`, `docs/Prompts/PruebaFlujoV1.txt`.
- **Archivos prohibidos**: Modificación en directorios de código (`frontend/`, `backend/`), migraciones y `docs/Logs.md`.
- **Criterio de éxito**: Asegurar que cada agente asuma de forma limpia su cuota de responsabilidad elevando dudas a este Agent Manager ante zonas grises.

### UXDesigner
- **Flujo de usuario**: El usuario pulsa sobre el CTA "Publicar", rellena el formulario de la vista, hace clic en "Subir anuncio" y recibe feedback, redireccionándose al detalle de dicho producto recién creado.
- **Pantallas necesarias**: Vista `Publicar Producto` (preferiblemente un formulario en pantalla completa para móviles o un modal interactivo en escritorio).
- **Campos del formulario**: Título, Descripción, Precio, Categoría y Selector de Imágenes (limitado a 3).
- **Validaciones visuales básicas**: El botón de envío debe estar inhabilitado (disabled) si el Título o la Categoría no se han completado o si el precio tiene caracteres no válidos. 
- **Estados**:
  - *Carga*: Spinner en botón tras pulsar.
  - *Error*: Mensaje de texto tipo toast en rojo (`Revise los campos requeridos`).
  - *Éxito*: Notificación verde y animación de redirección.
- **Recomendaciones para RustFrontend**: Usar componentes estándar ya existentes. El formulario debe ser 100% responsivo y claro.
- **Dudas para el Agent Manager**: ¿Permitimos la publicación de artículos gratuitos estableciendo el precio en 0?

### RustArchitectAgent
- **Capas implicadas**: API Router, Handlers y Services en backend; Views y API Services en frontend.
- **Módulos participantes**: `product_routes`, `product_service`, `product_repository` en RustBackend.
- **Separación de responsabilidades**: El frontend es el cliente que envía DTOs purificados (JSON); el backend asume la validación estricta y delegación de capa, el repository abstrae SQL.
- **Documentación a revisar**: `docs/API_STRUCTURE.md` para el nuevo endpoint.
- **Riesgos arquitectónicos**: Que el frontend envíe imágenes en base64 en vez de `multipart/form-data`, sobrecargando la memoria JSON en Rust.

### DatabaseAgent
- **Tablas implicadas**: `products`.
- **Campos necesarios**: `id` (INTEGER PRIMARY KEY), `user_id` (INTEGER, FK), `title` (TEXT), `description` (TEXT), `price` (REAL), `category_id` (INTEGER, FK), `created_at` (DATETIME).
- **Relaciones**: Referencias cruzadas hacia `users` y `categories` con políticas `ON DELETE CASCADE`.
- **Validaciones de datos**: `CHECK(price >= 0)`. `title NOT NULL`.
- **Posibles índices**: Un índice en `user_id` para listar rápido los productos de un estudiante.
- **Coordinación con RustBackend**: Validar que el tipo `DATETIME` de SQLite será consumido correctamente por la librería Chrono de Rust.

### RustBackend
- **Endpoints necesarios**: `POST /api/products`.
- **Servicios implicados**: `ProductService::create`.
- **DTOs**: `CreateProductRequest` para entrada, `ProductResponse` para retorno.
- **Validaciones backend**: Comprobar vigencia de sesión (token valid). Asegurar que la categoría existe en BD antes de persistir.
- **Gestión de errores**:
  - `400 Bad Request` si los datos del formulario son incompletos.
  - `401 Unauthorized` si no existe token.
  - `500 Internal Server Error` si la inserción en DB falla.
- **Tests recomendados**: `test_create_product_success`, `test_create_product_invalid_price`.

### RustFrontend
- **Pantalla o componente**: Vista `PublishView` y componente `PublishForm`.
- **Formulario de publicación**: Implementación del diseño propuesto por UX.
- **Llamada a API**: `ProductApiService.create_product(payload)`.
- **Coordinación**: Con BackendRust para definir si las imágenes se envían por separado o juntas, y con UXDesigner para los textos de error.

### QAAgent
- **Casos de prueba funcionales**: Publicar producto válido, publicar con datos faltantes.
- **Casos backend**: Rechazo de payload malformado o sin autenticación.
- **Casos frontend**: Comprobación del botón inactivo cuando el form es inválido. Feedback visual tras timeout de red.
- **Casos DB**: Que un producto insertado posea su correspondiente clave de usuario foránea y un precio no negativo.
- **Checklist de aceptación**: Confirmar registro visible en la UI y estado de respuesta 201 en backend.

### SequrityAgent
- **Riesgos de seguridad**: IDOR (intentar publicar en nombre de otro usuario falsificando el user_id), XSS (texto de descripción con scripts), File Upload vulnerabilities.
- **Permisos necesarios**: Endpoint protegido por middleware de autenticación (requiere Bearer token válido).
- **Riesgos sobre imágenes**: Validar MIME types en el backend. Impedir ficheros que no sean imágenes o superen un tamaño razonable (e.g. 5MB).
- **Recomendaciones**: El backend nunca debe confiar en el `user_id` enviado por el frontend, sino derivarlo del token de autenticación.

### DevopsAgent
- **Variables de entorno**: Es posible que se necesite configurar `MAX_UPLOAD_SIZE` o `UPLOAD_DIRECTORY`.
- **Scripts**: Ningún cambio requerido en scripts de arranque a priori.
- **Impacto CI/CD**: Solo añadir ejecución de la nueva suite de tests del módulo products.
- **Riesgos DevOps**: Asegurar que la carpeta de alojamiento de ficheros estáticos está en `.gitignore`.

### DocumentationAgent
- **Documentos a actualizar**: `docs/API_DOCS.md` (Añadir `POST /api/products`), `docs/DATABASE_SCHEMA.md`.
- **Evidencias útiles**: Captura de pantalla de Postman con una inserción exitosa y sus logs en DB.
- **Resumen para Logs**: Documentar esta fase bajo el control de avance del proyecto.

### ReviewerAgent
- **Rol respetado**: Sí, cada agente ha circunscrito sus recomendaciones a su área de especialidad.
- **Jerarquía respetada**: Sí, UXDesigner delegó decisiones del negocio al Agent Manager y DevopsAgent no intervino en programación.
- **Código y normas**: Nadie rompió la regla de no implementar código ni redactar migraciones directas.
- **Riesgos detectados**: Múltiples agentes señalan el potencial problema de la subida de imágenes (peso, base64 vs multipart). Es un riesgo real que justifica la prueba.
- **Recomendación final**: Flujo válido con ajustes menores (clarificar la subida de imágenes).

## 6. Riesgos detectados
1. **Manejo de Imágenes**: Fricción de coordinación entre Frontend, Backend, Devops y Seguridad. Las imágenes implican un manejo de multipart, almacenamiento en servidor y comprobación de seguridad, siendo el paso más complejo de la publicación.

## 7. Problemas de coordinación detectados
No existen solapamientos estructurales, la arquitectura multi-agente funciona perfectamente segmentando especialidades. La duda de diseño ha sido correctamente escalada.

## 8. Conclusiones de la prueba
La prueba es altamente exitosa. El esquema garantiza una segmentación lógica de tareas sin incurrir en problemas clásicos de sobreingeniería. Todos los agentes asumen la directriz primordial de planificar y coordinar antes de programar, asegurando la escalabilidad del MVP.

## 9. Veredicto final del flujo
- Flujo válido para comenzar Fase 0.

## 10. Resumen para DocumentationAgent

```markdown
- **Fecha:** 2026-05-20
- **Agente responsable:** Agent Manager / Product Manager Agent.
- **Acción realizada:** Prueba de flujo de agentes PruebaFlujoV1.
- **Archivos creados o modificados:** `docs/Prompts/PruebaFlujoV1.txt`, `docs/AGENT_FLOW_TEST.md`.
- **Motivo:** Validar la coordinación entre agentes antes de comenzar a programar.
- **Funcionalidades mantenidas:** No se modifican funcionalidades reales, solo se prueba el flujo.
- **Próximo paso recomendado:** Iniciar la Fase 0 autorizando al RustArchitectAgent a definir la arquitectura base y la estructura de carpetas.
```
