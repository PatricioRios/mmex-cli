# Acciones / Inversiones (stocks)

Gestion de inversiones en acciones y fondos mutuos.

## Comandos Disponibles

### Listar todas las acciones

```bash
mmex --db base.mmb stocks list
```

### Obtener una accion por ID

```bash
mmex --db base.mmb stocks get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb stocks get 1
```

### Crear una nueva entrada de accion

```bash
mmex --db base.mmb stocks create <CUENTA_ID> <FECHA_COMPRA> <NOMBRE> <NUM_ACCIONES> <PRECIO_COMPRA> <PRECIO_ACTUAL> <VALOR> <COMISION> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `CUENTA_ID` | ID de la cuenta donde se mantiene la inversion |
| `FECHA_COMPRA` | Fecha de compra (formato: `YYYY-MM-DD`) |
| `NOMBRE` | Nombre de la accion o fondo |
| `NUM_ACCIONES` | Numero de acciones |
| `PRECIO_COMPRA` | Precio de compra por accion |
| `PRECIO_ACTUAL` | Precio actual por accion |
| `VALOR` | Valor total actual |
| `COMISION` | Comision pagada |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--symbol` | Simbolo ticker (ej: "AAPL") |
| `--notes` | Notas |

Ejemplo:

```bash
mmex --db base.mmb stocks create 3 "2023-06-15" "Apple Inc." "10" "180.00" "195.50" "1955.00" "9.99" \
  --symbol AAPL \
  --notes "Inversion inicial en Apple"
```

### Actualizar una accion (completa)

```bash
mmex --db base.mmb stocks update <ID> <CUENTA_ID> <FECHA_COMPRA> <NOMBRE> <NUM_ACCIONES> <PRECIO_COMPRA> <PRECIO_ACTUAL> <VALOR> <COMISION> [OPCIONES]
```

Las opciones son las mismas que para `create`.

Ejemplo:

```bash
mmex --db base.mmb stocks update 1 3 "2023-06-15" "Apple Inc." "10" "180.00" "200.00" "2000.00" "9.99" \
  --symbol AAPL
```

### Actualizar parcialmente una accion

```bash
mmex --db base.mmb stocks update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--held-at` | Nueva cuenta de inversion |
| `--purchase-date` | Nueva fecha de compra |
| `--name` | Nuevo nombre |
| `--num-shares` | Nuevo numero de acciones |
| `--purchase-price` | Nuevo precio de compra |
| `--current-price` | Nuevo precio actual |
| `--value` | Nuevo valor total |
| `--commission` | Nueva comision |
| `--symbol` | Nuevo simbolo ticker |
| `--notes` | Nuevas notas |

Ejemplo:

```bash
mmex --db base.mmb stocks update-partial 1 --current-price "210.00" --value "2100.00"
```

### Eliminar una accion

```bash
mmex --db base.mmb stocks delete <ID>
```

## Campos de Accion

| Campo | Descripcion |
|-------|-------------|
| `held_at` | ID de la cuenta de inversion asociada |
| `purchase_date` | Fecha de compra |
| `name` | Nombre de la accion o fondo |
| `num_shares` | Cantidad de acciones |
| `purchase_price` | Precio unitario de compra |
| `current_price` | Precio unitario actual |
| `value` | Valor total actual (num_shares * current_price) |
| `commission` | Comision de la operacion |
| `symbol` | Codigo ticker de la accion |

## Ejemplos con JSON

```bash
# Listar acciones en JSON
mmex --db base.mmb -j stocks list

# Obtener detalles en JSON
mmex --db base.mmb -j stocks get 1
```
