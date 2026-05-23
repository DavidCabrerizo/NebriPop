---
name: github-version-control
description: Skill especializada en control de versiones con Git y GitHub para revisar cambios del proyecto, detectar modificaciones relevantes, preparar commits claros, evitar subir archivos sensibles y sincronizar el repositorio oficial de NebriPop únicamente cuando el Agent Manager lo autorice.
---

# GitHub Version Control Skill

Skill especializada en control de versiones con Git y GitHub para revisar cambios del proyecto, detectar modificaciones relevantes, preparar commits claros, evitar subir archivos sensibles y sincronizar el repositorio oficial de NebriPop únicamente cuando el Agent Manager lo autorice.

## Objetivo principal
Gestionar el repositorio GitHub del proyecto NebriPop de forma ordenada, segura y documentada. Analizar el estado del repositorio, revisar cambios, proponer commits profesionales, comprobar el remoto, evitar errores de seguridad y preparar subidas a GitHub bajo supervisión del Agent Manager.

## Capacidades principales

- Revisar el estado del repositorio con `git status`.
- Revisar diferencias con `git diff` y `git diff --staged`.
- Verificar la rama actual con `git branch`.
- Revisar últimos commits con `git log --oneline`.
- Comprobar el remoto con `git remote -v`.
- Confirmar que el remoto apunta a: `https://github.com/DavidCabrerizo/NebriPop/`
- Detectar archivos nuevos, modificados o eliminados.
- Identificar cambios relevantes dignos de commit.
- Agrupar cambios por temática.
- Proponer mensajes de commit claros y profesionales.
- Evitar commits demasiado grandes o mezclados.
- Detectar archivos sensibles antes de hacer commit y revisar `.gitignore`.
- Recomendar ramas de trabajo si es necesario.
- Preparar commits y push solo cuando el Agent Manager lo autorice.
- Preparar resúmenes para DocumentationAgent.
- Coordinarse con otros agentes cuando proceda.

## Cambios dignos de commit
- Nuevas funcionalidades, corrección de errores, pruebas técnicas, documentación importante.
- Creación o actualización de agentes o skills.
- Cambios en backend, frontend, base de datos, arquitectura, QA, seguridad, DevOps.
- Cierre de fases del proyecto o avances importantes del MVP.

## Archivos que NO deben subirse (salvo autorización explícita)
- `.env`, `.env.local`
- Credenciales, tokens, claves privadas, archivos con contraseñas.
- Bases de datos locales con datos sensibles.
- Carpeta `target/`.
- Archivos temporales, de caché, logs locales innecesarios, capturas temporales.
- Archivos personales del sistema o generados automáticamente sin valor para el proyecto.

## Reglas de seguridad obligatorias
1. **Nunca** hacer commit sin autorización explícita del Agent Manager.
2. **Nunca** hacer push sin autorización explícita del Agent Manager.
3. **Nunca** hacer force push.
4. **Nunca** borrar ramas sin autorización.
5. **Nunca** reescribir el historial de Git sin autorización.
6. **Nunca** subir `.env`, credenciales, tokens, claves privadas o la carpeta `target/`.
7. Si se detecta un secreto, detener el proceso y avisar al Agent Manager.
8. Si `.gitignore` no protege archivos importantes, proponer corregirlo antes de hacer commit.
9. No modificar `docs/Logs.md` directamente salvo autorización explícita; normalmente debe preparar un resumen para DocumentationAgent.

## Formato de análisis antes de proponer commit
Al preparar un commit, debe devolver:
1. Rama actual.
2. Estado del repositorio.
3. Archivos nuevos.
4. Archivos modificados.
5. Archivos eliminados.
6. Archivos que no deben subirse.
7. Riesgos detectados.
8. Agrupación recomendada de cambios.
9. Mensaje de commit recomendado.
10. Agentes que deberían revisar antes del commit.
11. Solicitud de autorización al Agent Manager.

## Formato de análisis antes de proponer push
Al preparar un push, debe devolver:
1. Rama actual.
2. Commits pendientes de subir.
3. Repositorio remoto configurado.
4. Confirmación de que el remoto apunta a: `https://github.com/DavidCabrerizo/NebriPop/`
5. Riesgos detectados.
6. Confirmación de que no se subirán secretos.
7. Solicitud de autorización al Agent Manager.

## Formato recomendado de commits
La skill debe recomendar mensajes siguiendo este estilo:
- `feat: añade estructura inicial del MVP`
- `feat: implementa prueba pre-MVP con Leptos Axum y SQLite`
- `fix: corrige integración frontend-backend`
- `docs: documenta agentes del proyecto`
- `docs: añade prompt de prueba pre-MVP`
- `test: añade checklist QA de integración`
- `chore: configura repositorio GitHub`
- `refactor: mejora estructura del backend Rust`

## Coordinación con futuros agentes
- **Agent Manager / Product Manager Agent**: Para autorizar commits y push.
- **DocumentationAgent**: Para registrar cambios en `docs/Logs.md`.
- **ReviewerAgent**: Para revisar cambios importantes.
- **QAAgent**: Para validar pruebas antes de subir cambios funcionales.
- **DevopsAgent**: Para revisar cambios de CI/CD, scripts o despliegue.
- **SequrityAgent**: Para revisar secretos o riesgos de seguridad.
- **RustBackend**: Para cambios backend.
- **RustFrontend**: Para cambios frontend.
- **DatabaseAgent**: Para cambios SQLite.
- **RustArchitectAgent**: Para cambios arquitectónicos.

## Archivos que esta skill puede ayudar a revisar
- `.gitignore`
- `README.md`
- `AGENTS.md`
- `RULES.md`
- `AI_WORKFLOW.md`
- `docs/`
- `backend/`
- `frontend/`
- `database/`
- `experiments/`
- `.github/workflows/`
- `scripts/`

## Archivos que NO debe modificar sin autorización
- `PRD.md`
- `IMPLEMENTATION_PLAN.md`
- Código fuente del backend.
- Código fuente del frontend.
- Migraciones.
- Archivos de configuración críticos.
- `docs/Logs.md`
- Archivos fuera del alcance de la tarea.

## Primera tarea recomendada para un agente que use esta skill
Realizar una auditoría inicial del repositorio:
1. Comprobar si Git está inicializado.
2. Comprobar si existe remoto.
3. Comprobar si el remoto apunta a: `https://github.com/DavidCabrerizo/NebriPop/`
4. Comprobar rama actual.
5. Revisar archivos modificados.
6. Revisar archivos sin seguimiento.
7. Revisar `.gitignore`.
8. Detectar riesgos.
9. Recomendar siguiente commit.
10. **No hacer commit ni push todavía**.
