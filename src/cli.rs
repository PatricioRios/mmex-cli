use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "mmex")]
#[command(
    about = "CLI for Money Manager EX database operations",
    long_about = "A command-line interface for interacting with Money Manager EX (.mmb) databases.\nIt allows you to view, create, update, and delete various financial records\nsuch as accounts, transactions, categories, and more."
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        short,
        long,
        env = "MMEX_DB_PATH",
        help = "Path to the .mmb database file",
        global = true
    )]
    pub db: Option<String>,

    #[arg(
        short = 'k',
        long,
        env = "MMEX_DB_KEY",
        help = "Database encryption key"
    )]
    pub key: Option<String>,

    #[arg(short = 'j', long, help = "Output in JSON format")]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage bank accounts, credit cards, and other financial accounts
    #[command(subcommand)]
    Accounts(AccountCommands),

    /// Manage income, expenses, and transfer transactions
    #[command(subcommand)]
    Transactions(TransactionCommands),

    /// Manage transaction categories and subcategories
    #[command(subcommand)]
    Categories(CategoryCommands),

    /// Manage payees (people or institutions you pay or receive money from)
    #[command(subcommand)]
    Payees(PayeeCommands),

    /// Manage currencies and exchange rates
    #[command(subcommand)]
    Currencies(CurrencyCommands),

    /// Manage tags for categorizing transactions
    #[command(subcommand)]
    Tags(TagCommands),

    /// Manage assets (properties, vehicles, etc.)
    #[command(subcommand)]
    Assets(AssetCommands),

    /// Manage stock and mutual fund investments
    #[command(subcommand)]
    Stocks(StockCommands),

    /// Manage recurring or scheduled transactions
    #[command(subcommand)]
    Scheduled(ScheduledCommands),

    /// Support operations like getting/setting database settings and version info
    #[command(subcommand)]
    Support(SupportCommands),

    /// Generate shell completions for the CLI
    Completions {
        /// The shell to generate completions for
        shell: Shell,
    },

    /// Print the version of the CLI
    Version,
}

#[derive(Subcommand)]
pub enum AccountCommands {
    /// List all accounts
    List,
    /// Get details of a specific account
    Get { id: i64 },
    /// Get the balance of a specific account
    Balance { id: i64 },
    /// Create a new account
    Create {
        name: String,
        account_type: String,
        initial_balance: String,
        currency_id: i64,
        #[arg(long)]
        account_num: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        favorite: Option<bool>,
    },
    /// Update an existing account completely
    Update {
        id: i64,
        name: String,
        account_type: String,
        initial_balance: String,
        currency_id: i64,
        #[arg(long)]
        account_num: Option<String>,
        #[arg(long)]
        status: String,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        favorite: bool,
    },
    /// Update specific fields of an existing account
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        account_type: Option<String>,
        #[arg(long)]
        initial_balance: Option<String>,
        #[arg(long)]
        currency_id: Option<i64>,
        #[arg(long)]
        account_num: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        favorite: Option<bool>,
    },
    /// Delete an account
    Delete { id: i64 },
}

#[derive(Subcommand)]
pub enum TransactionCommands {
    /// List all transactions
    List {
        #[arg(long)]
        account_id: Option<i64>,
    },
    /// Get details of a specific transaction
    Get { id: i64 },
    /// Create a new transaction
    Create {
        account_id: i64,
        payee_id: i64,
        trans_code: String,
        amount: String,
        status: String,
        #[arg(long)]
        to_account_id: Option<i64>,
        #[arg(long)]
        transaction_number: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        date: Option<String>,
        #[arg(long)]
        to_amount: Option<String>,
    },
    /// Update an existing transaction completely
    Update {
        id: i64,
        account_id: i64,
        payee_id: i64,
        trans_code: String,
        amount: String,
        status: String,
        #[arg(long)]
        to_account_id: Option<i64>,
        #[arg(long)]
        transaction_number: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        date: Option<String>,
        #[arg(long)]
        to_amount: Option<String>,
    },
    /// Update specific fields of an existing transaction
    UpdatePartial {
        id: i64,
        #[arg(long)]
        account_id: Option<i64>,
        #[arg(long)]
        payee_id: Option<i64>,
        #[arg(long)]
        trans_code: Option<String>,
        #[arg(long)]
        amount: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        to_account_id: Option<i64>,
        #[arg(long)]
        transaction_number: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        date: Option<String>,
        #[arg(long)]
        to_amount: Option<String>,
    },
    /// Delete a transaction
    Delete { id: i64 },
    /// Get tags linked to a transaction
    GetTags { id: i64 },
    /// Link a tag to a transaction
    LinkTag { id: i64, tag_id: i64 },
    /// Unlink a tag from a transaction
    UnlinkTag { id: i64, tag_id: i64 },
    /// Get splits for a transaction
    GetSplits { id: i64 },
    /// Add a split to a transaction
    AddSplit {
        transaction_id: i64,
        amount: String,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Update an existing transaction split
    UpdateSplit {
        id: i64,
        transaction_id: i64,
        amount: String,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Delete a transaction split
    DeleteSplit { id: i64 },
}

#[derive(Subcommand)]
pub enum CategoryCommands {
    /// List all categories
    List,
    /// Get details of a specific category
    Get { id: i64 },
    /// List subcategories for a given parent category
    Subcategories { parent_id: i64 },
    /// Create a new category
    Create {
        name: String,
        #[arg(long)]
        parent_id: Option<i64>,
    },
    /// Update an existing category completely
    Update {
        id: i64,
        name: String,
        #[arg(action = clap::ArgAction::Set)]
        active: bool,
        #[arg(long)]
        parent_id: Option<i64>,
    },
    /// Update specific fields of an existing category
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        active: Option<bool>,
        #[arg(long)]
        parent_id: Option<i64>,
    },
    /// Delete a category
    Delete { id: i64 },
}

#[derive(Subcommand)]
pub enum PayeeCommands {
    /// List all payees
    List,
    /// Get details of a specific payee
    Get { id: i64 },
    /// Create a new payee
    Create { name: String },
    /// Update an existing payee completely
    Update {
        id: i64,
        name: String,
        #[arg(action = clap::ArgAction::Set)]
        active: bool,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        number: Option<String>,
        #[arg(long)]
        website: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        pattern: Option<String>,
    },
    /// Update specific fields of an existing payee
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        active: Option<bool>,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        number: Option<String>,
        #[arg(long)]
        website: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        pattern: Option<String>,
    },
    /// Delete a payee
    Delete { id: i64 },
}

#[derive(Subcommand)]
pub enum CurrencyCommands {
    /// List all currencies
    List,
    /// Get details of a specific currency
    Get { id: i64 },
    /// Get a currency by its symbol
    BySymbol { symbol: String },
    /// Create a new currency
    Create {
        name: String,
        symbol: String,
        currency_type: String,
        scale: i32,
        base_conv_rate: String,
        #[arg(long)]
        pfx_symbol: Option<String>,
        #[arg(long)]
        sfx_symbol: Option<String>,
        #[arg(long)]
        decimal_point: Option<String>,
        #[arg(long)]
        group_separator: Option<String>,
        #[arg(long)]
        unit_name: Option<String>,
        #[arg(long)]
        cent_name: Option<String>,
    },
    /// Update an existing currency completely
    Update {
        id: i64,
        name: String,
        symbol: String,
        currency_type: String,
        scale: i32,
        base_conv_rate: String,
        #[arg(long)]
        pfx_symbol: Option<String>,
        #[arg(long)]
        sfx_symbol: Option<String>,
        #[arg(long)]
        decimal_point: Option<String>,
        #[arg(long)]
        group_separator: Option<String>,
        #[arg(long)]
        unit_name: Option<String>,
        #[arg(long)]
        cent_name: Option<String>,
    },
    /// Update specific fields of an existing currency
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        symbol: Option<String>,
        #[arg(long)]
        currency_type: Option<String>,
        #[arg(long)]
        scale: Option<i32>,
        #[arg(long)]
        base_conv_rate: Option<String>,
        #[arg(long)]
        pfx_symbol: Option<String>,
        #[arg(long)]
        sfx_symbol: Option<String>,
        #[arg(long)]
        decimal_point: Option<String>,
        #[arg(long)]
        group_separator: Option<String>,
        #[arg(long)]
        unit_name: Option<String>,
        #[arg(long)]
        cent_name: Option<String>,
    },
    /// Delete a currency
    Delete { id: i64 },
}

#[derive(Subcommand)]
pub enum TagCommands {
    /// List all tags
    List,
    /// Get details of a specific tag
    Get { id: i64 },
    /// Create a new tag
    Create { name: String },
    /// Update an existing tag
    Update { id: i64, name: String },
    /// Update specific fields of an existing tag
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete a tag
    Delete { id: i64 },
    /// Get tags for a specific reference (transaction, etc.)
    GetForReference { ref_type: String, ref_id: i64 },
    /// Link a tag to a reference
    LinkToReference {
        ref_type: String,
        ref_id: i64,
        tag_id: i64,
    },
    /// Unlink a tag from a reference
    UnlinkFromReference {
        ref_type: String,
        ref_id: i64,
        tag_id: i64,
    },
}

#[derive(Subcommand)]
pub enum AssetCommands {
    /// List all assets
    List,
    /// Get details of a specific asset
    Get { id: i64 },
    /// Create a new asset
    Create {
        name: String,
        start_date: String,
        status: String,
        value: String,
        #[arg(long)]
        currency_id: Option<i64>,
        #[arg(long)]
        value_change_mode: Option<String>,
        #[arg(long)]
        value_change: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long, default_value = "0.0")]
        value_change_rate: f64,
        #[arg(long)]
        asset_type: Option<String>,
    },
    /// Update an existing asset completely
    Update {
        id: i64,
        name: String,
        start_date: String,
        status: String,
        value: String,
        #[arg(long)]
        currency_id: Option<i64>,
        #[arg(long)]
        value_change_mode: Option<String>,
        #[arg(long)]
        value_change: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long, default_value = "0.0")]
        value_change_rate: f64,
        #[arg(long)]
        asset_type: Option<String>,
    },
    /// Update specific fields of an existing asset
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        value: Option<String>,
        #[arg(long)]
        currency_id: Option<i64>,
        #[arg(long)]
        value_change_mode: Option<String>,
        #[arg(long)]
        value_change: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        value_change_rate: Option<f64>,
        #[arg(long)]
        asset_type: Option<String>,
    },
    /// Delete an asset
    Delete { id: i64 },
}

#[derive(Subcommand)]
pub enum StockCommands {
    /// List all stocks
    List,
    /// Get details of a specific stock
    Get { id: i64 },
    /// Create a new stock entry
    Create {
        held_at: i64,
        purchase_date: String,
        name: String,
        num_shares: String,
        purchase_price: String,
        current_price: String,
        value: String,
        commission: String,
        #[arg(long)]
        symbol: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Update an existing stock entry completely
    Update {
        id: i64,
        held_at: i64,
        purchase_date: String,
        name: String,
        num_shares: String,
        purchase_price: String,
        current_price: String,
        value: String,
        commission: String,
        #[arg(long)]
        symbol: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Update specific fields of an existing stock entry
    UpdatePartial {
        id: i64,
        #[arg(long)]
        held_at: Option<i64>,
        #[arg(long)]
        purchase_date: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        num_shares: Option<String>,
        #[arg(long)]
        purchase_price: Option<String>,
        #[arg(long)]
        current_price: Option<String>,
        #[arg(long)]
        value: Option<String>,
        #[arg(long)]
        commission: Option<String>,
        #[arg(long)]
        symbol: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Delete a stock entry
    Delete { id: i64 },
}

#[derive(Subcommand)]
pub enum ScheduledCommands {
    /// List all scheduled transactions
    List,
    /// Get details of a specific scheduled transaction
    Get { id: i64 },
    /// Create a new scheduled transaction
    Create {
        account_id: i64,
        payee_id: i64,
        trans_code: String,
        amount: String,
        status: String,
        repeats: i32,
        num_occurrences: i32,
        #[arg(long)]
        to_account_id: Option<i64>,
        #[arg(long)]
        transaction_number: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        trans_date: Option<String>,
        #[arg(long)]
        next_occurrence_date: Option<String>,
        #[arg(long)]
        to_trans_amount: Option<String>,
    },
    /// Update an existing scheduled transaction completely
    Update {
        id: i64,
        account_id: i64,
        payee_id: i64,
        trans_code: String,
        amount: String,
        status: String,
        repeats: i32,
        num_occurrences: i32,
        #[arg(long)]
        to_account_id: Option<i64>,
        #[arg(long)]
        transaction_number: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        trans_date: Option<String>,
        #[arg(long)]
        next_occurrence_date: Option<String>,
        #[arg(long)]
        to_trans_amount: Option<String>,
    },
    /// Update specific fields of an existing scheduled transaction
    UpdatePartial {
        id: i64,
        #[arg(long)]
        account_id: Option<i64>,
        #[arg(long)]
        payee_id: Option<i64>,
        #[arg(long)]
        trans_code: Option<String>,
        #[arg(long)]
        amount: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        repeats: Option<i32>,
        #[arg(long)]
        num_occurrences: Option<i32>,
        #[arg(long)]
        to_account_id: Option<i64>,
        #[arg(long)]
        transaction_number: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        trans_date: Option<String>,
        #[arg(long)]
        next_occurrence_date: Option<String>,
        #[arg(long)]
        to_trans_amount: Option<String>,
    },
    /// Delete a scheduled transaction
    Delete { id: i64 },
}

#[derive(Subcommand)]
pub enum SupportCommands {
    /// Get the database schema version
    DbVersion,
    /// Get a specific database setting by name
    GetSetting { name: String },
    /// Set a specific database setting
    SetSetting { name: String, value: String },
}
