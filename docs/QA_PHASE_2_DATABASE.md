# Checklist de Validación de Base de Datos (QAAgent)

## Verificaciones Generales
- [x] El archivo `schema.sql` se ejecuta sin errores de sintaxis.
- [x] El archivo `seeds.sql` se ejecuta e inserta las filas correctamente.
- [x] Las claves primarias están definidas (AUTOINCREMENT).
- [x] Las claves foráneas están configuradas.

## Restricciones y Validaciones (CHECKs comprobados lógicamente)
- [x] Imposible insertar categorías duplicadas (UNIQUE name).
- [x] Imposible insertar usuarios con email duplicado (UNIQUE email).
- [x] No se puede insertar un producto con precio negativo (CHECK price >= 0).
- [x] `status` restringido a valores específicos (available, reserved, sold).
- [x] `condition` restringida a los estados de desgaste definidos.

## Comandos de Prueba
Puedes verificar manualmente el entorno con:
```bash
sqlite3 database/nebripop.db < database/schema.sql
sqlite3 database/nebripop.db < database/seeds.sql
sqlite3 database/nebripop.db ".tables"
sqlite3 database/nebripop.db "SELECT * FROM categories;"
sqlite3 database/nebripop.db "SELECT title, price, location FROM products;"
```
