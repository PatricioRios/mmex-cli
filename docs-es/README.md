# Documentacion en Espanol - MMEX CLI

Bienvenido a la documentacion en espanol de `mmex_cli`, la interfaz de linea de comandos para interactuar con bases de datos de Money Manager EX (`.mmb`).

## Tabla de Contenidos

### Guia de Inicio

| Documento | Descripcion |
|-----------|-------------|
| [Instalacion](instalacion.md) | Como instalar `mmex_cli` en tu sistema |
| [Uso General](uso-general.md) | Flags globales, variables de entorno y formatos de salida |

### Referencia de Comandos

| Documento | Descripcion |
|-----------|-------------|
| [Cuentas](cuentas.md) | Gestion de cuentas bancarias, tarjetas de credito y otras cuentas financieras |
| [Transacciones](transacciones.md) | Gestion de ingresos, gastos, transferencias y transacciones divididas |
| [Categorias](categorias.md) | Gestion de categorias y subcategorias de transacciones |
| [Beneficiarios](beneficiarios.md) | Gestion de beneficiarios (personas o instituciones) |
| [Monedas](monedas.md) | Gestion de monedas y tasas de cambio |
| [Etiquetas](etiquetas.md) | Gestion de etiquetas para categorizar transacciones |
| [Activos](activos.md) | Gestion de activos (propiedades, vehiculos, etc.) |
| [Acciones](acciones.md) | Gestion de inversiones en acciones y fondos mutuos |
| [Transacciones Programadas](programadas.md) | Gestion de transacciones recurrentes o programadas |
| [Soporte](soporte.md) | Operaciones de soporte: version de BD, configuracion |

## Vision General

`mmex_cli` es una herramienta de linea de comandos escrita en Rust que permite leer, crear, actualizar y eliminar datos financieros almacenados en tu base de datos MMEX sin necesidad de abrir la aplicacion grafica.

### Entidades Soportadas

El CLI soporta las siguientes entidades financieras:

- **Cuentas** (`accounts`) - Cuentas bancarias, tarjetas de credito, etc.
- **Transacciones** (`transactions`) - Ingresos, gastos y transferencias
- **Categorias** (`categories`) - Clasificacion de transacciones
- **Beneficiarios** (`payees`) - Personas o instituciones que pagan o reciben dinero
- **Monedas** (`currencies`) - Monedas y tasas de cambio
- **Etiquetas** (`tags`) - Etiquetas para categorizar transacciones
- **Activos** (`assets`) - Propiedades, vehiculos, etc.
- **Acciones** (`stocks`) - Inversiones en acciones y fondos mutuos
- **Transacciones Programadas** (`scheduled`) - Transacciones recurrentes
- **Soporte** (`support`) - Configuracion y version de la base de datos

### Operaciones CRUD

Cada entidad soporta las operaciones basicas:

| Operacion | Descripcion |
|-----------|-------------|
| `list` | Listar todos los registros |
| `get <ID>` | Obtener detalles de un registro especifico |
| `create` | Crear un nuevo registro |
| `update` | Actualizar un registro completo |
| `update-partial` | Actualizar campos especificos de un registro |
| `delete <ID>` | Eliminar un registro |

## Licencia

Este proyecto esta licenciado bajo MIT.
