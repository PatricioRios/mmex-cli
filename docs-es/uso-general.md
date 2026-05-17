# Uso General

## Sintaxis Basica

```bash
mmex [FLAGS] <COMANDO> [SUBCOMANDO] [ARGUMENTOS]
```

## Flags Globales

Estos flags se aplican a todos los comandos:

| Flag | Abreviatura | Descripcion |
|------|-------------|-------------|
| `--db <RUTA>` | `-d` | **(Requerido)** Ruta al archivo de base de datos `.mmb`. Tambien se puede configurar con la variable de entorno `MMEX_DB_PATH`. |
| `--key <CLAVE>` | `-k` | **(Opcional)** Clave de encriptacion si tu archivo `.mmb` esta encriptado. Tambien se puede configurar con `MMEX_DB_KEY`. |
| `--json` | `-j` | **(Opcional)** Formatea la salida como JSON en lugar de tablas legibles. Util para scripts e integraciones. |
| `--help` | `-h` | Muestra la ayuda del comando. |
| `--version` | `-V` | Muestra la version del CLI. |

## Variables de Entorno

Para mayor comodidad, especialmente al escribir scripts, puedes exportar la ruta de tu base de datos y la clave como variables de entorno:

```bash
export MMEX_DB_PATH="/ruta/a/tu/base_de_datos.mmb"
export MMEX_DB_KEY="tu-clave-secreta"  # solo si esta encriptada
```

Una vez configuradas, puedes ejecutar comandos sin especificar `--db`:

```bash
mmex accounts list
mmex -j transactions list
```

## Formatos de Salida

### Tabla (predeterminado)

Por defecto, los resultados se muestran en tablas legibles:

```bash
mmex --db database.mmb accounts list
```

Salida de ejemplo:

```
ID  Nombre                 Tipo        Moneda  Favorita
1   Cuenta de Ahorro       Checking    USD     No
2   Tarjeta de Credito     Credit      USD     Si
3   Cuenta de Inversiones  Investment  USD     No
```

### JSON

Usa el flag `-j` o `--json` para obtener la salida en formato JSON:

```bash
mmex --db database.mmb -j accounts list
```

Salida de ejemplo:

```json
[
  {
    "id": 1,
    "name": "Cuenta de Ahorro",
    "account_type": "Checking",
    "currency_id": 1,
    "favorite": false
  }
]
```

Este formato es ideal para integrar con herramientas como `jq`, scripts de Python, o cualquier herramienta de procesamiento de datos.

## Comandos Especiales

### Ver la Version

```bash
mmex version
```

### Generar Autocompletado

```bash
mmex completions <shell>
```

Donde `<shell>` puede ser: `bash`, `zsh`, `fish`, `elvish`, `powershell`.

## Convenciones de los Comandos

### Argumentos Posicionales

Los argumentos posicionales se especifican sin nombre y en un orden fijo:

```bash
mmex --db db.mmb accounts get 1
#                                   ^^ ID de la cuenta (posicional)
```

### Opciones Nombradas

Las opciones nombradas usan `--nombre-valor` y se pueden colocar en cualquier orden:

```bash
mmex --db db.mmb transactions create 1 2 Withdrawal 50.00 Reconciled \
  --notes "Compras del supermercado" \
  --date "2023-10-25"
```

### Tipos de Operacion

| Comando | Descripcion |
|---------|-------------|
| `list` | Lista todos los registros. Algunas entidades aceptan filtros. |
| `get <ID>` | Obtiene los detalles de un registro por su ID. |
| `create` | Crea un nuevo registro con los argumentos proporcionados. |
| `update` | Actualiza un registro completo (requiere todos los campos). |
| `update-partial` | Actualiza solo los campos especificados de un registro. |
| `delete <ID>` | Elimina un registro por su ID. |
