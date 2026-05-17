# Cuentas (accounts)

Gestion de cuentas bancarias, tarjetas de credito y otras cuentas financieras.

## Comandos Disponibles

### Listar todas las cuentas

```bash
mmex --db base.mmb accounts list
```

Muestra una tabla con todas las cuentas registradas en la base de datos.

### Obtener una cuenta por ID

```bash
mmex --db base.mmb accounts get <ID>
```

Ejemplo:

```bash
mmex --db base.mmb accounts get 1
```

### Obtener el saldo de una cuenta

```bash
mmex --db base.mmb accounts balance <ID>
```

Ejemplo:

```bash
mmex --db base.mmb accounts balance 1
```

### Crear una nueva cuenta

```bash
mmex --db base.mmb accounts create <NOMBRE> <TIPO> <SALDO_INICIAL> <MONEDA_ID> [OPCIONES]
```

**Argumentos posicionales:**

| Argumento | Descripcion |
|-----------|-------------|
| `NOMBRE` | Nombre de la cuenta |
| `TIPO` | Tipo de cuenta (ej: `Checking`, `Credit`, `Investment`) |
| `SALDO_INICIAL` | Saldo inicial de la cuenta |
| `MONEDA_ID` | ID de la moneda asociada |

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--account-num` | Numero de cuenta |
| `--status` | Estado de la cuenta |
| `--notes` | Notas o descripcion |
| `--favorite` | Marcar como favorita (`true`/`false`) |

Ejemplo:

```bash
mmex --db base.mmb accounts create "Mi Cuenta de Ahorro" Checking 1000.00 1 --favorite true --notes "Cuenta principal"
```

### Actualizar una cuenta (completa)

```bash
mmex --db base.mmb accounts update <ID> <NOMBRE> <TIPO> <SALDO_INICIAL> <MONEDA_ID> [OPCIONES]
```

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--account-num` | Numero de cuenta |
| `--status` | Estado de la cuenta (requerido) |
| `--notes` | Notas o descripcion |
| `--favorite` | Marcar como favorita (`true`/`false`, requerido) |

Ejemplo:

```bash
mmex --db base.mmb accounts update 1 "Cuenta Actualizada" Checking 1500.00 1 --status Open --favorite true
```

### Actualizar parcialmente una cuenta

```bash
mmex --db base.mmb accounts update-partial <ID> [OPCIONES]
```

Solo se actualizaran los campos proporcionados.

**Opciones:**

| Opcion | Descripcion |
|--------|-------------|
| `--name` | Nuevo nombre |
| `--account-type` | Nuevo tipo de cuenta |
| `--initial-balance` | Nuevo saldo inicial |
| `--currency-id` | Nueva moneda |
| `--account-num` | Nuevo numero de cuenta |
| `--status` | Nuevo estado |
| `--notes` | Nuevas notas |
| `--favorite` | Cambiar estado de favorita |

Ejemplo:

```bash
mmex --db base.mmb accounts update-partial 1 --name "Nuevo Nombre" --notes "Notas actualizadas"
```

### Eliminar una cuenta

```bash
mmex --db base.mmb accounts delete <ID>
```

Ejemplo:

```bash
mmex --db base.mmb accounts delete 1
```

## Tipos de Cuenta

Los tipos de cuenta comunes en MMEX incluyen:

| Tipo | Descripcion |
|------|-------------|
| `Checking` | Cuenta corriente o de ahorro |
| `Credit` | Tarjeta de credito |
| `Investment` | Cuenta de inversion |
| `Term` | Deposito a plazo |
| `Cash` | Efectivo |

## Ejemplos de Uso con JSON

```bash
# Listar cuentas en formato JSON
mmex --db base.mmb -j accounts list

# Obtener detalles en JSON
mmex --db base.mmb -j accounts get 1
```
