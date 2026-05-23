# Test de Leptos: Calculadora Básica

## Objetivo de la prueba
Validar la capacidad técnica del equipo y la viabilidad de **Leptos** como framework de frontend para el proyecto NebriPop, creando una calculadora interactiva básica con reactividad.

## Motivo de probar Leptos
NebriPop está estructurado para usar Rust tanto en backend como en frontend. Leptos permite crear aplicaciones web reactivas con Rust utilizando WASM, manteniendo el código seguro, rápido y compartiendo tipos con el backend (cuando sea necesario). Esta prueba permite evaluar su curva de aprendizaje y herramientas.

## Agentes que han intervenido
- **Agent Manager / Product Manager Agent**: Coordinación del flujo y definición del alcance.
- **RustArchitectAgent**: Planificación de la estructura de la prueba (carpeta separada en `experiments/leptos_calculator/`).
- **RustFrontend**: Implementación de la calculadora (estado reactivo, inputs y botones).
- **QAAgent**: Definición de los casos de prueba y chequeos límite.
- **ReviewerAgent**: Validación del código (Clean Code, buenas prácticas, aislamiento).
- **DocumentationAgent**: Creación de la documentación y preparación del resumen final.

## Implementación
Se ha creado un crate binario en `experiments/leptos_calculator` configurado para compilar a WebAssembly utilizando `trunk`.
El código cuenta con:
- Dos cajas de texto vinculadas a señales (`create_signal`) para `num1` y `num2`.
- Una señal para el `resultado`.
- Botones que ejecutan cierres (closures) encargados de parsear los inputs a `f64` y aplicar las operaciones: Suma, Resta, Multiplicación, División.
- Manejo explícito del error por división entre cero, reflejándolo visualmente.

## Archivos Creados
- `docs/Prompts/PruebaLeptosCalculadoraV1.txt`
- `docs/QA_LEPTOS_CALCULADORA.md`
- `docs/REVIEW_LEPTOS_CALCULADORA.md`
- `docs/LEPTOS_CALCULADORA_TEST.md`
- `experiments/leptos_calculator/Cargo.toml`
- `experiments/leptos_calculator/index.html`
- `experiments/leptos_calculator/src/main.rs`

## Cómo ejecutar la prueba
Para ejecutar este experimento, se necesita tener instalado Rust, el target de wasm y la herramienta `trunk`.

1. Instalar el target WebAssembly (si no está instalado):
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Instalar Trunk:
   ```bash
   cargo install trunk
   ```
3. Navegar al directorio y ejecutar el servidor de desarrollo:
   ```bash
   cd experiments/leptos_calculator
   trunk serve --open
   ```

## Resultado esperado
Una página web simple que muestre dos campos de entrada, botones de operaciones matemáticas (+, -, *, /) y una etiqueta con el resultado. Si se divide por cero, muestra un error en la etiqueta del resultado. Todo el flujo es reactivo.

## Conclusiones
La prueba ha demostrado que Leptos ofrece un sistema de reactividad basado en señales muy parecido a SolidJS, permitiendo crear UI directamente en Rust sin gran verbosidad. La separación de responsabilidades y la compatibilidad con Trunk hace que el entorno de desarrollo sea ágil. La prueba se considera un éxito y valida a Leptos como candidato para la interfaz del proyecto NebriPop.

---

### Resumen preparado para registrar en docs/Logs.md
*(A la espera de autorización del Agent Manager)*

- **Fecha:** 21 de mayo de 2026.
- **Agente responsable:** DocumentationAgent (orquestado por Agent Manager).
- **Acción realizada:** Creación de una prueba técnica aislada (Calculadora básica) usando el framework Leptos.
- **Archivos creados o modificados:**
  - `docs/Prompts/PruebaLeptosCalculadoraV1.txt`
  - `docs/LEPTOS_CALCULADORA_TEST.md`
  - `docs/QA_LEPTOS_CALCULADORA.md`
  - `docs/REVIEW_LEPTOS_CALCULADORA.md`
  - `experiments/leptos_calculator/*` (Cargo.toml, src/main.rs, index.html)
- **Motivo del cambio:** Validar el uso de Leptos como framework frontend reactivo en Rust antes del desarrollo del MVP de NebriPop.
- **Funcionalidades implementadas:** Interfaz web interactiva con operaciones de suma, resta, multiplicación y división, con protección frente a la división por cero.
- **Próximo paso recomendado:** Proceder a planificar el andamiaje del proyecto principal de NebriPop usando Leptos tras esta validación técnica exitosa.
