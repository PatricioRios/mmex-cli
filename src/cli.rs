use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mmex")]
#[command(about = "CLI for Money Manager EX database operations", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        short,
        long,
        env = "MMEX_DB_PATH",
        help = "Path to the .mmb database file"
    )]
    pub db: String,

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
    #[command(subcommand)]
    Accounts(AccountCommands),

    #[command(subcommand)]
    Transactions(TransactionCommands),

    #[command(subcommand)]
    Categories(CategoryCommands),

    #[command(subcommand)]
    Payees(PayeeCommands),

    #[command(subcommand)]
    Currencies(CurrencyCommands),

    #[command(subcommand)]
    Tags(TagCommands),

    #[command(subcommand)]
    Assets(AssetCommands),

    #[command(subcommand)]
    Stocks(StockCommands),

    #[command(subcommand)]
    Scheduled(ScheduledCommands),

    #[command(subcommand)]
    Support(SupportCommands),

    Version,
}

#[derive(Subcommand)]
pub enum AccountCommands {
    List,
    Get {
        id: i64,
    },
    Balance {
        id: i64,
    },
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
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum TransactionCommands {
    List {
        #[arg(long)]
        account_id: Option<i64>,
    },
    Get {
        id: i64,
    },
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
    Delete {
        id: i64,
    },
    GetTags {
        id: i64,
    },
    LinkTag {
        id: i64,
        tag_id: i64,
    },
    UnlinkTag {
        id: i64,
        tag_id: i64,
    },
    GetSplits {
        id: i64,
    },
    AddSplit {
        transaction_id: i64,
        amount: String,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        notes: Option<String>,
    },
    UpdateSplit {
        id: i64,
        transaction_id: i64,
        amount: String,
        #[arg(long)]
        category_id: Option<i64>,
        #[arg(long)]
        notes: Option<String>,
    },
    DeleteSplit {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum CategoryCommands {
    List,
    Get {
        id: i64,
    },
    Subcategories {
        parent_id: i64,
    },
    Create {
        name: String,
        #[arg(long)]
        parent_id: Option<i64>,
    },
    Update {
        id: i64,
        name: String,
        active: bool,
        #[arg(long)]
        parent_id: Option<i64>,
    },
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        active: Option<bool>,
        #[arg(long)]
        parent_id: Option<i64>,
    },
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum PayeeCommands {
    List,
    Get {
        id: i64,
    },
    Create {
        name: String,
    },
    Update {
        id: i64,
        name: String,
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
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum CurrencyCommands {
    List,
    Get {
        id: i64,
    },
    BySymbol {
        symbol: String,
    },
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
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    List,
    Get {
        id: i64,
    },
    Create {
        name: String,
    },
    Update {
        id: i64,
        name: String,
    },
    UpdatePartial {
        id: i64,
        #[arg(long)]
        name: Option<String>,
    },
    Delete {
        id: i64,
    },
    GetForReference {
        ref_type: String,
        ref_id: i64,
    },
    LinkToReference {
        ref_type: String,
        ref_id: i64,
        tag_id: i64,
    },
    UnlinkFromReference {
        ref_type: String,
        ref_id: i64,
        tag_id: i64,
    },
}

#[derive(Subcommand)]
pub enum AssetCommands {
    List,
    Get {
        id: i64,
    },
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
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum StockCommands {
    List,
    Get {
        id: i64,
    },
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
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum ScheduledCommands {
    List,
    Get {
        id: i64,
    },
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
    Delete {
        id: i64,
    },
}

#[derive(Subcommand)]
pub enum SupportCommands {
    DbVersion,
    GetSetting { name: String },
    SetSetting { name: String, value: String },
}
