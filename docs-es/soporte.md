# Soporte (support)

Operaciones de soporte: version de la base de datos, configuracion y ajustes.

## Comandos Disponibles

### Obtener la version del esquema de la base de datos

```bash
mmex --db base.mmb support db-version
```

Muestra la version del esquema de la base de datos MMEX.

### Obtener una configuracion de la base de datos

```bash
mmex --db base.mmb support get-setting <NOMBRE>
```

Obtiene el valor de una configuracion especifica de la base de datos.

Ejemplo:

```bash
mmex --db base.mmb support get-setting "USERNAME"
```

### Establecer una configuracion de la base de datos

```bash
mmex --db base.mmb support set-setting <NOMBRE> <VALOR>
```

Establece o actualiza el valor de una configuracion en la base de datos.

Ejemplo:

```bash
mmex --db base.mmb support set-setting "USERNAME" "Patricio"
```

## Configuraciones Comunes

Algunas configuraciones que se pueden consultar o modificar:

| Nombre | Descripcion |
|--------|-------------|
| `USERNAME` | Nombre de usuario de la base de datos |
| `DATEFORMAT` | Formato de fecha |
| `BASECURRENCYID` | ID de la moneda base |
| `DATABASEVERSION` | Version del esquema |

## Ejemplos con JSON

```bash
# Obtener la version en JSON
mmex --db base.mmb -j support db-version

# Obtener una configuracion en JSON
mmex --db base.mmb -j support get-setting "USERNAME"
```
