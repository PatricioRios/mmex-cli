# Monedas (currencies)

Gestion de monedas y tasas de cambio.

## Comandos Disponibles

### Listar todas las monedas

```bash
mmex --db base.mmb currencies list
```

### Obtener una moneda por ID

```bash
mmex --db base.mmb currencies get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb currencies get 1
```

### Obtener una moneda por simbolo

```bash
mmex --db base.mmb currencies by-symbol <SIMBOLO>
```

Ejemplo:

```bash
mmex --db base.mmb currencies by-symbol USD
```

### Crear una nueva moneda

```bash
mmex --db base.mmb currencies create <NOMBRE> <SIMBOLO> <TIPO> <ESCALA> <TASA_BASE> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `NOMBRE` | Nombre de la moneda (ej: "US Dollar") |
| `SIMBOLO` | Simbolo de la moneda (ej: "USD") |
| `TIPO` | Tipo de moneda (ej: "Fiat") |
| `ESCALA` | Escala decimal (ej: 100 para 2 decimales) |
| `TASA_BASE` | Tasa de conversion base |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--pfx-symbol` | Simbolo prefijo (ej: "$") |
| `--sfx-symbol` | Simbolo sufijo |
| `--decimal-point` | Separador decimal (ej: ".") |
| `--group-separator` | Separador de miles (ej: ",") |
| `--unit-name` | Nombre de la unidad (ej: "Dollar") |
| `--cent-name` | Nombre del centavo (ej: "Cent") |

Ejemplo:

```bash
mmex --db base.mmb currencies create "Peso Chileno" "CLP" "Fiat" 1 "0.0012" \
  --pfx-symbol "$" \
  --decimal-point "," \
  --group-separator "."
```

### Actualizar una moneda (completa)

```bash
mmex --db base.mmb currencies update <ID> <NOMBRE> <SIMBOLO> <TIPO> <ESCALA> <TASA_BASE> [OPCIONES]
```

Las opciones son las mismas que para `create`.

Ejemplo:

```bash
mmex --db base.mmb currencies update 1 "US Dollar" "USD" "Fiat" 100 "1.0" \
  --pfx-symbol "$" \
  --decimal-point "." \
  --group-separator ","
```

### Actualizar parcialmente una moneda

```bash
mmex --db base.mmb currencies update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--name` | Nuevo nombre |
| `--symbol` | Nuevo simbolo |
| `--currency-type` | Nuevo tipo |
| `--scale` | Nueva escala |
| `--base-conv-rate` | Nueva tasa de conversion |
| `--pfx-symbol` | Nuevo simbolo prefijo |
| `--sfx-symbol` | Nuevo simbolo sufijo |
| `--decimal-point` | Nuevo separador decimal |
| `--group-separator` | Nuevo separador de miles |
| `--unit-name` | Nuevo nombre de unidad |
| `--cent-name` | Nuevo nombre de centavo |

Ejemplo:

```bash
mmex --db base.mmb currencies update-partial 2 --base-conv-rate "0.0011"
```

### Eliminar una moneda

```bash
mmex --db base.mmb currencies delete <ID>
```

## Campos de Moneda

| Campo | Descripcion | Ejemplo |
|-------|-------------|---------|
| `name` | Nombre completo | "US Dollar" |
| `symbol` | Codigo ISO | "USD" |
| `currency_type` | Tipo de moneda | "Fiat" |
| `scale` | Escala (decimales) | 100 = 2 decimales |
| `base_conv_rate` | Tasa respecto a la moneda base | "1.0" |
| `pfx_symbol` | Simbolo que aparece antes del monto | "$" |
| `sfx_symbol` | Simbolo que aparece despues del monto | "EUR" |
| `decimal_point` | Separador decimal | "." |
| `group_separator` | Separador de miles | "," |
| `unit_name` | Nombre de la unidad | "Dollar" |
| `cent_name` | Nombre de la fraccion | "Cent" |

## Ejemplos con JSON

```bash
# Listar monedas en JSON
mmex --db base.mmb -j currencies list

# Buscar por simbolo en JSON
mmex --db base.mmb -j currencies by-symbol USD
```
