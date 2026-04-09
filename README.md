# MMEX CLI

`mmex_cli` is a command-line interface (CLI) for interacting directly with Money Manager EX (MMEX) `.mmb` database files. It provides a simple and efficient way to read, create, update, and delete financial data stored in your MMEX database without needing to open the graphical application.

## Installation

You can install `mmex_cli` directly using `cargo` if you have Rust installed on your system:

```bash
cargo install mmex_cli
```

Alternatively, you can clone the repository and build it manually:

```bash
cargo build --release
```
The executable will be located in `target/release/mmex`.

## Global Flags

When using the `mmex` command, there are a few global flags available to configure the CLI behavior:

- `-d, --db <DB>`: (Required) Specifies the path to your `.mmb` database file. You can also set this using the `MMEX_DB_PATH` environment variable.
- `-k, --key <KEY>`: (Optional) Provides the database encryption key if your `.mmb` file is encrypted. This can also be provided via the `MMEX_DB_KEY` environment variable.
- `-j, --json`: (Optional) Formats the output as JSON instead of human-readable text tables. This is highly useful for scripting and integration with other tools.
- `-h, --help`: Prints the help information for a given command.
- `-V, --version`: Prints the CLI version.

## Usage & Examples

Here are some examples of the most common operations you can perform with the CLI.

### Working with Accounts

**List all accounts:**
```bash
mmex --db /path/to/database.mmb accounts list
```
*To get output in JSON format, append the `-j` flag:*
```bash
mmex --db /path/to/database.mmb -j accounts list
```

**Get a specific account by ID:**
```bash
mmex --db /path/to/database.mmb accounts get 1
```

**Partially update an account (e.g., rename it):**
```bash
mmex --db /path/to/database.mmb accounts update-partial 1 --name "My New Savings Account"
```

### Working with Transactions

**List all transactions:**
```bash
mmex --db /path/to/database.mmb transactions list
```

**Create a new transaction:**
The `create` command requires specific arguments in order: `<ACCOUNT_ID> <PAYEE_ID> <TRANS_CODE> <AMOUNT> <STATUS>`. 
*Note: Depending on your database setup, these ID values need to exist.*
```bash
# Example: create a transaction for account 1, payee 2, code 'Withdrawal', amount 50.00, and status 'Reconciled'
mmex --db /path/to/database.mmb transactions create 1 2 Withdrawal 50.00 Reconciled --notes "Grocery shopping" --date "2023-10-25"
```

### Other Entities
The CLI also supports other entities like categories, payees, currencies, tags, assets, stocks, and scheduled transactions. You can explore these by using the help flag:
```bash
mmex --help
mmex categories --help
mmex payees --help
```

## Environment Variables

For convenience, especially when writing scripts, you can export your database path and key as environment variables so you don't have to specify them every time:

```bash
export MMEX_DB_PATH="/path/to/your/database.mmb"
# export MMEX_DB_KEY="your-secret-key" (if encrypted)

# Now you can run commands simply:
mmex accounts list
mmex -j transactions list
```