# Beneficiarios (payees)

Gestion de beneficiarios: personas o instituciones de las que recibes o a las que envias dinero.

## Comandos Disponibles

### Listar todos los beneficiarios

```bash
mmex --db base.mmb payees list
```

### Obtener un beneficiario por ID

```bash
mmex --db base.mmb payees get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb payees get 3
```

### Crear un nuevo beneficiario

```bash
mmex --db base.mmb payees create <NOMBRE>
```

Ejemplo:

```bash
mmex --db base.mmb payees create "Supermercado Central"
```

### Actualizar un beneficiario (completo)

```bash
mmex --db base.mmb payees update <ID> <NOMBRE> <ACTIVO> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `ID` | ID del beneficiario |
| `NOMBRE` | Nuevo nombre |
| `ACTIVO` | Estado activo (`true` o `false`) |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--category-id` | ID de la categoria predeterminada |
| `--number` | Numero de referencia o telefono |
| `--website` | Sitio web |
| `--notes` | Notas |
| `--pattern` | Patron para coincidencia automatica |

Ejemplo:

```bash
mmex --db base.mmb payees update 3 "Supermercado Central" true --category-id 5 --notes "Supermercado principal"
```

### Actualizar parcialmente un beneficiario

```bash
mmex --db base.mmb payees update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--name` | Nuevo nombre |
| `--active` | Nuevo estado activo |
| `--category-id` | Nueva categoria predeterminada |
| `--number` | Nuevo numero |
| `--website` | Nuevo sitio web |
| `--notes` | Nuevas notas |
| `--pattern` | Nuevo patron |

Ejemplo:

```bash
mmex --db base.mmb payees update-partial 3 --notes "Notas actualizadas"
```

### Eliminar un beneficiario

```bash
mmex --db base.mmb payees delete <ID>
```

Ejemplo:

```bash
mmex --db base.mmb payees delete 3
```

## Patrones de Coincidencia

Los beneficiarios pueden tener un `pattern` que permite la asignacion automatica de categorias cuando se importan transacciones. El patron es una expresion regular que se compara con la descripcion de la transaccion.

```bash
mmex --db base.mmb payees update-partial 3 --pattern "SUPERMERCADO.*CENTRAL"
```

## Ejemplos con JSON

```bash
# Listar beneficiarios en JSON
mmex --db base.mmb -j payees list

# Obtener detalles en JSON
mmex --db base.mmb -j payees get 3
```
