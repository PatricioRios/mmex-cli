# Activos (assets)

Gestion de activos: propiedades, vehiculos, maquinaria y otros bienes de valor.

## Comandos Disponibles

### Listar todos los activos

```bash
mmex --db base.mmb assets list
```

### Obtener un activo por ID

```bash
mmex --db base.mmb assets get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb assets get 1
```

### Crear un nuevo activo

```bash
mmex --db base.mmb assets create <NOMBRE> <FECHA_INICIO> <ESTADO> <VALOR> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `NOMBRE` | Nombre del activo |
| `FECHA_INICIO` | Fecha de adquisicion (formato: `YYYY-MM-DD`) |
| `ESTADO` | Estado del activo (ej: `Open`, `Closed`) |
| `VALOR` | Valor del activo |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--currency-id` | ID de la moneda |
| `--value-change-mode` | Modo de cambio de valor: `Percentage`, `Linear` |
| `--value-change` | Tipo de cambio de valor: `None`, `Appreciate`, `Depreciate` |
| `--notes` | Notas o descripcion |
| `--value-change-rate` | Tasa de cambio de valor (predeterminado: 0.0) |
| `--asset-type` | Tipo de activo: `Property`, `Automobile`, `Household Object`, `Art`, `Jewellery`, `Cash`, `Other` |

Ejemplo:

```bash
mmex --db base.mmb assets create "Departamento Centro" "2020-01-15" Open "150000.00" \
  --currency-id 1 \
  --asset-type Property \
  --value-change Appreciate \
  --value-change-mode Percentage \
  --value-change-rate 3.0 \
  --notes "Departamento de 2 dormitorios"
```

### Actualizar un activo (completo)

```bash
mmex --db base.mmb assets update <ID> <NOMBRE> <FECHA_INICIO> <ESTADO> <VALOR> [OPCIONES]
```

Las opciones son las mismas que para `create`.

Ejemplo:

```bash
mmex --db base.mmb assets update 1 "Departamento Centro" "2020-01-15" Open "175000.00" \
  --currency-id 1 \
  --asset-type Property
```

### Actualizar parcialmente un activo

```bash
mmex --db base.mmb assets update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--name` | Nuevo nombre |
| `--start-date` | Nueva fecha de inicio |
| `--status` | Nuevo estado |
| `--value` | Nuevo valor |
| `--currency-id` | Nueva moneda |
| `--value-change-mode` | Nuevo modo de cambio |
| `--value-change` | Nuevo tipo de cambio |
| `--notes` | Nuevas notas |
| `--value-change-rate` | Nueva tasa de cambio |
| `--asset-type` | Nuevo tipo de activo |

Ejemplo:

```bash
mmex --db base.mmb assets update-partial 1 --value "180000.00" --notes "Valor actualizado 2024"
```

### Eliminar un activo

```bash
mmex --db base.mmb assets delete <ID>
```

## Tipos de Activo

| Tipo | Descripcion |
|------|-------------|
| `Property` | Propiedad raiz |
| `Automobile` | Vehiculo |
| `Household Object` | Objeto del hogar |
| `Art` | Obra de arte |
| `Jewellery` | Joyeria |
| `Cash` | Efectivo |
| `Other` | Otro |

## Modos de Cambio de Valor

| Modo | Descripcion |
|------|-------------|
| `Percentage` | Cambio porcentual anual |
| `Linear` | Cambio lineal (monto fijo) |

## Tipos de Cambio de Valor

| Tipo | Descripcion |
|------|-------------|
| `None` | Sin cambio |
| `Appreciate` | Apreciacion (aumenta) |
| `Depreciate` | Depreciacion (disminuye) |

## Ejemplos con JSON

```bash
# Listar activos en JSON
mmex --db base.mmb -j assets list

# Obtener detalles en JSON
mmex --db base.mmb -j assets get 1
```
