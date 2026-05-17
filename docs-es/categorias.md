# Categorias (categories)

Gestion de categorias y subcategorias para clasificar transacciones.

## Comandos Disponibles

### Listar todas las categorias

```bash
mmex --db base.mmb categories list
```

### Obtener una categoria por ID

```bash
mmex --db base.mmb categories get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb categories get 5
```

### Listar subcategorias

```bash
mmex --db base.mmb categories subcategories <PARENT_ID>
```

Lista todas las subcategorias que pertenecen a una categoria padre.

Ejemplo:

```bash
mmex --db base.mmb categories subcategories 1
```

### Crear una nueva categoria

```bash
mmex --db base.mmb categories create <NOMBRE> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `NOMBRE` | Nombre de la categoria |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--parent-id` | ID de la categoria padre (para crear subcategorias) |

Ejemplos:

```bash
# Crear una categoria raiz
mmex --db base.mmb categories create "Alimentacion"

# Crear una subcategoria
mmex --db base.mmb categories create "Restaurantes" --parent-id 1
```

### Actualizar una categoria (completa)

```bash
mmex --db base.mmb categories update <ID> <NOMBRE> <ACTIVO> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `ID` | ID de la categoria |
| `NOMBRE` | Nuevo nombre |
| `ACTIVO` | Estado activo (`true` o `false`) |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--parent-id` | ID de la categoria padre |

Ejemplo:

```bash
mmex --db base.mmb categories update 5 "Comida" true
```

### Actualizar parcialmente una categoria

```bash
mmex --db base.mmb categories update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--name` | Nuevo nombre |
| `--active` | Nuevo estado activo |
| `--parent-id` | Nueva categoria padre |

Ejemplo:

```bash
mmex --db base.mmb categories update-partial 5 --name "Alimentacion y Bebidas"
```

### Eliminar una categoria

```bash
mmex --db base.mmb categories delete <ID>
```

Ejemplo:

```bash
mmex --db base.mmb categories delete 5
```

## Estructura de Categorias

Las categorias en MMEX siguen una estructura jerarquica:

```
Alimentacion (ID: 1)
  ├── Restaurantes (ID: 10, parent_id: 1)
  ├── Supermercado (ID: 11, parent_id: 1)
  └── Cafe (ID: 12, parent_id: 1)
Transporte (ID: 2)
  ├── Gasolina (ID: 20, parent_id: 2)
  └── Transporte Publico (ID: 21, parent_id: 2)
```

- Las categorias padre tienen `parent_id = null`
- Las subcategorias referencian a su categoria padre mediante `parent_id`

## Ejemplos con JSON

```bash
# Listar categorias en JSON
mmex --db base.mmb -j categories list

# Obtener subcategorias en JSON
mmex --db base.mmb -j categories subcategories 1
```
