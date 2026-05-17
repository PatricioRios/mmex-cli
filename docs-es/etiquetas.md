# Etiquetas (tags)

Gestion de etiquetas para categorizar y organizar transacciones.

## Comandos Disponibles

### Listar todas las etiquetas

```bash
mmex --db base.mmb tags list
```

### Obtener una etiqueta por ID

```bash
mmex --db base.mmb tags get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb tags get 1
```

### Crear una nueva etiqueta

```bash
mmex --db base.mmb tags create <NOMBRE>
```

Ejemplo:

```bash
mmex --db base.mmb tags create "Deducible"
mmex --db base.mmb tags create "Negocio"
mmex --db base.mmb tags create "Personal"
```

### Actualizar una etiqueta

```bash
mmex --db base.mmb tags update <ID> <NOMBRE>
```

Ejemplo:

```bash
mmex --db base.mmb tags update 1 "Deducible de Impuestos"
```

### Actualizar parcialmente una etiqueta

```bash
mmex --db base.mmb tags update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--name` | Nuevo nombre |

Ejemplo:

```bash
mmex --db base.mmb tags update-partial 1 --name "Gasto Deducible"
```

### Eliminar una etiqueta

```bash
mmex --db base.mmb tags delete <ID>
```

### Obtener etiquetas por referencia

```bash
mmex --db base.mmb tags get-for-reference <TIPO_REFERENCIA> <REF_ID>
```

Obtiene las etiquetas vinculadas a una referencia especifica (por ejemplo, una transaccion).

Ejemplo:

```bash
mmex --db base.mmb tags get-for-reference Transaction 42
```

### Vincular una etiqueta a una referencia

```bash
mmex --db base.mmb tags link-to-reference <TIPO_REFERENCIA> <REF_ID> <ETIQUETA_ID>
```

Ejemplo:

```bash
mmex --db base.mmb tags link-to-reference Transaction 42 1
```

### Desvincular una etiqueta de una referencia

```bash
mmex --db base.mmb tags unlink-from-reference <TIPO_REFERENCIA> <REF_ID> <ETIQUETA_ID>
```

Ejemplo:

```bash
mmex --db base.mmb tags unlink-from-reference Transaction 42 1
```

## Tipos de Referencia

Los tipos de referencia comunes son:

| Tipo | Descripcion |
|------|-------------|
| `Transaction` | Transacciones |

## Flujo de Trabajo Tipico

```bash
# 1. Crear etiquetas
mmex --db base.mmb tags create "Deducible"
mmex --db base.mmb tags create "Negocio"

# 2. Vincular etiquetas a transacciones
mmex --db base.mmb tags link-to-reference Transaction 42 1
mmex --db base.mmb tags link-to-reference Transaction 42 2

# 3. Ver etiquetas de una transaccion
mmex --db base.mmb tags get-for-reference Transaction 42

# 4. Desvincular si es necesario
mmex --db base.mmb tags unlink-from-reference Transaction 42 1
```

## Ejemplos con JSON

```bash
# Listar etiquetas en JSON
mmex --db base.mmb -j tags list

# Obtener etiquetas de una transaccion en JSON
mmex --db base.mmb -j tags get-for-reference Transaction 42
```
