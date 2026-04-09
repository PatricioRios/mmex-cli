# Funcionalidades Implementadas en el CLI (mmex_cli)

Este documento registra el progreso de implementación de funcionalidades en el CLI de Money Manager EX (`mmex_cli`), comparándolo con las capacidades de la biblioteca subyacente `mmex_lib`.

## Estado Actual

El CLI actualmente expone las siguientes funcionalidades de solo lectura:

- **Accounts:** List, Get, Balance
- **Transactions:** List, Get
- **Categories:** List, Get, Subcategories
- **Payees:** List, Get
- **Currencies:** List, Get, BySymbol
- **Tags:** List, Get
- **Assets:** List, Get
- **Stocks:** List, Get
- **Scheduled:** List, Get

## Plan de Implementación (Funcionalidades Faltantes)

Se irán marcando como completadas a medida que se implementen en el CLI.

### 1. Support
- [x] `get_db_version`
- [x] `get_setting`
- [x] `set_setting`

### 2. Accounts
- [x] `create_account`
- [x] `update_account`
- [x] `update_account_partial`
- [x] `delete_account`

### 3. Categories
- [x] `create_category`
- [x] `update_category`
- [x] `update_category_partial`
- [x] `delete_category`

### 4. Payees
- [x] `create_payee`
- [x] `update_payee`
- [x] `update_payee_partial`
- [x] `delete_payee`

### 5. Currencies
- [x] `create_currency`
- [x] `update_currency`
- [x] `update_currency_partial`
- [x] `delete_currency`

### 6. Tags
- [x] `create_tag`
- [x] `update_tag`
- [x] `update_tag_partial`
- [x] `delete_tag`
- [x] `get_for_reference`
- [x] `link_to_reference`
- [x] `unlink_from_reference`

### 7. Assets
- [x] `create_asset`
- [x] `update_asset`
- [x] `update_asset_partial`
- [x] `delete_asset`

### 8. Stocks
- [x] `create_stock`
- [x] `update_stock`
- [x] `update_stock_partial`
- [x] `delete_stock`

### 9. Scheduled
- [x] `create_scheduled`
- [x] `update_scheduled`
- [x] `update_scheduled_partial`
- [x] `delete_scheduled`

### 10. Transactions
- [x] `create_transaction`
- [x] `update_transaction`
- [x] `update_transaction_partial`
- [x] `delete_transaction`
- [x] `get_tags_for_transaction`
- [x] `link_tag`
- [x] `unlink_tag`
- [x] `get_splits_for_transaction`
- [x] `add_split`
- [x] `update_split`
- [x] `delete_split`
