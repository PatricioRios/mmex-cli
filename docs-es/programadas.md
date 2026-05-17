# Transacciones Programadas (scheduled)

Gestion de transacciones recurrentes o programadas.

## Comandos Disponibles

### Listar todas las transacciones programadas

```bash
mmex --db base.mmb scheduled list
```

### Obtener una transaccion programada por ID

```bash
mmex --db base.mmb scheduled get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb scheduled get 1
```

### Crear una nueva transaccion programada

```bash
mmex --db base.mmb scheduled create <CUENTA_ID> <BENEFICIARIO_ID> <CODIGO> <MONTO> <ESTADO> <REPETICION> <NUM_OCURRENCIAS> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `CUENTA_ID` | ID de la cuenta asociada |
| `BENEFICIARIO_ID` | ID del beneficiario |
| `CODIGO` | Tipo de transaccion: `Withdrawal`, `Deposit`, `Transfer` |
| `MONTO` | Monto de la transaccion |
| `ESTADO` | Estado: `Reconciled`, `Void`, `FollowUp`, `Duplicate` |
| `REPETICION` | Tipo de repeticion (ver tabla abajo) |
| `NUM_OCURRENCIAS` | Numero de ocurrencias (-1 para infinito) |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--to-account-id` | ID de la cuenta destino (para transferencias) |
| `--transaction-number` | Numero de referencia |
| `--notes` | Notas |
| `--category-id` | ID de la categoria |
| `--trans-date` | Fecha de la transaccion |
| `--next-occurrence-date` | Fecha de la proxima ocurrencia |
| `--to-trans-amount` | Monto en la cuenta destino |

Ejemplo:

```bash
mmex --db base.mmb scheduled create 1 2 Withdrawal "1500.00" Reconciled 4 -1 \
  --notes "Arriendo mensual" \
  --category-id 10 \
  --next-occurrence-date "2024-01-01"
```

### Actualizar una transaccion programada (completa)

```bash
mmex --db base.mmb scheduled update <ID> <CUENTA_ID> <BENEFICIARIO_ID> <CODIGO> <MONTO> <ESTADO> <REPETICION> <NUM_OCURRENCIAS> [OPCIONES]
```

Las opciones son las mismas que para `create`.

### Actualizar parcialmente una transaccion programada

```bash
mmex --db base.mmb scheduled update-partial <ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--account-id` | Nueva cuenta |
| `--payee-id` | Nuevo beneficiario |
| `--trans-code` | Nuevo tipo de transaccion |
| `--amount` | Nuevo monto |
| `--status` | Nuevo estado |
| `--repeats` | Nuevo tipo de repeticion |
| `--num-occurrences` | Nuevo numero de ocurrencias |
| `--to-account-id` | Nueva cuenta destino |
| `--transaction-number` | Nuevo numero de referencia |
| `--notes` | Nuevas notas |
| `--category-id` | Nueva categoria |
| `--trans-date` | Nueva fecha |
| `--next-occurrence-date` | Nueva fecha de proxima ocurrencia |
| `--to-trans-amount` | Nuevo monto destino |

Ejemplo:

```bash
mmex --db base.mmb scheduled update-partial 1 --amount "1600.00"
```

### Eliminar una transaccion programada

```bash
mmex --db base.mmb scheduled delete <ID>
```

## Tipos de Repeticion

| Valor | Descripcion |
|-------|-------------|
| 0 | Una sola vez |
| 1 | Semanal |
| 2 | Bisemanal |
| 3 | Mensual |
| 4 | Bimensual |
| 5 | Trimestral |
| 6 | Semestral |
| 7 | Anual |
| 8 | Cuatro meses |
| 9 | Cuatro semanas |
| 10 | Diariamente |
| 11 | Cada X dias (requiere informacion adicional) |
| 12 | Cada X meses (requiere informacion adicional) |

## Numero de Ocurrencias

| Valor | Descripcion |
|-------|-------------|
| `-1` | Repetir indefinidamente |
| `> 0` | Repetir N veces y luego desactivar |

## Ejemplos con JSON

```bash
# Listar transacciones programadas en JSON
mmex --db base.mmb -j scheduled list

# Obtener detalles en JSON
mmex --db base.mmb -j scheduled get 1
```
