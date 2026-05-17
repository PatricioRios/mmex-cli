# Transacciones (transactions)

Gestion de ingresos, gastos, transferencias y transacciones divididas.

## Comandos Disponibles

### Listar transacciones

```bash
mmex --db base.mmb transactions list [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--account-id <ID>` | Filtrar transacciones por ID de cuenta |

Ejemplo:

```bash
# Listar todas las transacciones
mmex --db base.mmb transactions list

# Filtrar por cuenta
mmex --db base.mmb transactions list --account-id 1
```

### Obtener una transaccion por ID

```bash
mmex --db base.mmb transactions get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb transactions get 42
```

### Crear una nueva transaccion

```bash
mmex --db base.mmb transactions create <CUENTA_ID> <BENEFICIARIO_ID> <CODIGO> <MONTO> <ESTADO> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `CUENTA_ID` | ID de la cuenta asociada |
| `BENEFICIARIO_ID` | ID del beneficiario |
| `CODIGO` | Tipo de transaccion: `Withdrawal`, `Deposit`, `Transfer` |
| `MONTO` | Monto de la transaccion |
| `ESTADO` | Estado: `Reconciled`, `Void`, `FollowUp`, `Duplicate` |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--to-account-id` | ID de la cuenta destino (para transferencias) |
| `--transaction-number` | Numero de transaccion o referencia |
| `--notes` | Notas de la transaccion |
| `--category-id` | ID de la categoria |
| `--date` | Fecha de la transaccion (formato: `YYYY-MM-DD`) |
| `--to-amount` | Monto en la cuenta destino (para transferencias) |

Ejemplo:

```bash
mmex --db base.mmb transactions create 1 2 Withdrawal 50.00 Reconciled \
  --notes "Compras del supermercado" \
  --date "2023-10-25" \
  --category-id 5
```

### Actualizar una transaccion (completa)

```bash
mmex --db base.mmb transactions update <ID> <CUENTA_ID> <BENEFICIARIO_ID> <CODIGO> <MONTO> <ESTADO> [OPCIONES]
```

Las opciones son las mismas que para `create`.

Ejemplo:

```bash
mmex --db base.mmb transactions update 42 1 2 Withdrawal 75.00 Reconciled \
  --notes "Monto actualizado" \
  --date "2023-10-26"
```

### Actualizar parcialmente una transaccion

```bash
mmex --db base.mmb transactions update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--account-id` | Cambiar la cuenta |
| `--payee-id` | Cambiar el beneficiario |
| `--trans-code` | Cambiar el tipo de transaccion |
| `--amount` | Cambiar el monto |
| `--status` | Cambiar el estado |
| `--to-account-id` | Cambiar la cuenta destino |
| `--transaction-number` | Cambiar el numero de referencia |
| `--notes` | Cambiar las notas |
| `--category-id` | Cambiar la categoria |
| `--date` | Cambiar la fecha |
| `--to-amount` | Cambiar el monto destino |

Ejemplo:

```bash
mmex --db base.mmb transactions update-partial 42 --notes "Notas actualizadas" --amount "100.00"
```

### Eliminar una transaccion

```bash
mmex --db base.mmb transactions delete <ID>
```

## Etiquetas de Transacciones

### Ver etiquetas de una transaccion

```bash
mmex --db base.mmb transactions get-tags <ID>
```

### Vincular una etiqueta

```bash
mmex --db base.mmb transactions link-tag <TRANSACCION_ID> <ETIQUETA_ID>
```

### Desvincular una etiqueta

```bash
mmex --db base.mmb transactions unlink-tag <TRANSACCION_ID> <ETIQUETA_ID>
```

## Transacciones Divididas (Splits)

Las transacciones divididas permiten asignar una transaccion a multiples categorias.

### Ver divisiones de una transaccion

```bash
mmex --db base.mmb transactions get-splits <ID>
```

### Agregar una division

```bash
mmex --db base.mmb transactions add-split <TRANSACCION_ID> <MONTO> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--category-id` | ID de la categoria |
| `--notes` | Notas de la division |

Ejemplo:

```bash
mmex --db base.mmb transactions add-split 42 25.00 --category-id 3 --notes "Comida"
mmex --db base.mmb transactions add-split 42 25.00 --category-id 7 --notes "Bebidas"
```

### Actualizar una division

```bash
mmex --db base.mmb transactions update-split <DIVISION_ID> <TRANSACCION_ID> <MONTO> [OPCIONES]
```

### Eliminar una division

```bash
mmex --db base.mmb transactions delete-split <DIVISION_ID>
```

## Codigos de Transaccion

| Codigo | Descripcion |
|--------|-------------|
| `Withdrawal` | Retiro / Gasto |
| `Deposit` | Deposito / Ingreso |
| `Transfer` | Transferencia entre cuentas |

## Estados de Transaccion

| Estado | Descripcion |
|--------|-------------|
| `Reconciled` | Reconciliada |
| `Void` | Anulada |
| `FollowUp` | Pendiente de seguimiento |
| `Duplicate` | Duplicada |

## Ejemplos con JSON

```bash
# Listar transacciones en JSON
mmex --db base.mmb -j transactions list

# Listar transacciones de una cuenta en JSON
mmex --db base.mmb -j transactions list --account-id 1

# Obtener divisiones en JSON
mmex --db base.mmb -j transactions get-splits 42
```
