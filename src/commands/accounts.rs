use crate::cli::AccountCommands;
use crate::output::{format_bool, format_option, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::accounts::{Account, AccountStatus, AccountType, AccountUpdate};
use mmex_lib::domain::types::{AccountId, CurrencyId, Money};
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &AccountCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        AccountCommands::List => list(ctx, format),
        AccountCommands::Get { id } => get(ctx, *id, format),
        AccountCommands::Balance { id } => balance(ctx, *id, format),
        AccountCommands::Create {
            name,
            account_type,
            initial_balance,
            currency_id,
            account_num,
            status,
            notes,
            favorite,
        } => create(
            ctx,
            name,
            account_type,
            initial_balance,
            *currency_id,
            account_num.clone(),
            status.clone(),
            notes.clone(),
            *favorite,
            format,
        ),
        AccountCommands::Update {
            id,
            name,
            account_type,
            initial_balance,
            currency_id,
            account_num,
            status,
            notes,
            favorite,
        } => update(
            ctx,
            *id,
            name,
            account_type,
            initial_balance,
            *currency_id,
            account_num.clone(),
            status.clone(),
            notes.clone(),
            *favorite,
            format,
        ),
        AccountCommands::UpdatePartial {
            id,
            name,
            account_type,
            initial_balance,
            currency_id,
            account_num,
            status,
            notes,
            favorite,
        } => update_partial(
            ctx,
            *id,
            name.clone(),
            account_type.clone(),
            initial_balance.clone(),
            *currency_id,
            account_num.clone(),
            status.clone(),
            notes.clone(),
            *favorite,
            format,
        ),
        AccountCommands::Delete { id } => delete(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let accounts = ctx.accounts().get_all_accounts()?;

    match format {
        OutputFormat::Json => print_json(&accounts),
        OutputFormat::Table => {
            let headers = ["ID", "Name", "Type", "Status", "Currency", "Favorite"];
            let rows: Vec<Vec<String>> = accounts
                .iter()
                .map(|a| {
                    vec![
                        a.id.v1.to_string(),
                        a.name.clone(),
                        a.account_type.to_string(),
                        a.status.to_string(),
                        a.currency_id.v1.to_string(),
                        format_bool(a.favorite).to_string(),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let account = ctx
        .accounts()
        .get_account_by_id(mmex_lib::domain::types::AccountId::new(id))?;

    match account {
        Some(a) => match format {
            OutputFormat::Json => print_json(&a),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), a.id.v1.to_string()],
                    vec!["Name".to_string(), a.name.clone()],
                    vec!["Type".to_string(), a.account_type.to_string()],
                    vec!["Status".to_string(), a.status.to_string()],
                    vec![
                        "Account Num".to_string(),
                        format_option(a.account_num.clone()),
                    ],
                    vec!["Notes".to_string(), format_option(a.notes.clone())],
                    vec!["Initial Balance".to_string(), a.initial_balance.v1.clone()],
                    vec!["Currency ID".to_string(), a.currency_id.v1.to_string()],
                    vec!["Favorite".to_string(), format_bool(a.favorite).to_string()],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Account not found"),
    }
    Ok(())
}

fn balance(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let balance = ctx
        .accounts()
        .get_account_balance(mmex_lib::domain::types::AccountId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&balance),
        OutputFormat::Table => {
            let headers = ["Field", "Value"];
            let rows = vec![
                vec!["Account ID".to_string(), balance.account_id.v1.to_string()],
                vec![
                    "Initial Balance".to_string(),
                    balance.initial_balance.v1.clone(),
                ],
                vec![
                    "Total Deposits".to_string(),
                    balance.total_deposits.v1.clone(),
                ],
                vec![
                    "Total Withdrawals".to_string(),
                    balance.total_withdrawals.v1.clone(),
                ],
                vec![
                    "Current Balance".to_string(),
                    balance.current_balance.v1.clone(),
                ],
            ];
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn create(
    ctx: &MmexContext,
    name: &str,
    account_type: &str,
    initial_balance: &str,
    currency_id: i64,
    account_num: Option<String>,
    status: Option<String>,
    notes: Option<String>,
    favorite: Option<bool>,
    format: OutputFormat,
) -> Result<()> {
    let account = Account {
        id: AccountId::new(0), // Will be ignored on insert
        name: name.to_string(),
        account_type: AccountType::from(account_type.to_string()),
        account_num,
        status: status
            .map(|s| AccountStatus::from(s))
            .unwrap_or(AccountStatus::Open),
        notes,
        initial_balance: Money {
            v1: initial_balance.to_string(),
        },
        currency_id: CurrencyId::new(currency_id),
        favorite: favorite.unwrap_or(false),
    };

    let created = ctx.accounts().create_account(&account)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Account created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(
    ctx: &MmexContext,
    id: i64,
    name: &str,
    account_type: &str,
    initial_balance: &str,
    currency_id: i64,
    account_num: Option<String>,
    status: String,
    notes: Option<String>,
    favorite: bool,
    format: OutputFormat,
) -> Result<()> {
    let account = Account {
        id: AccountId::new(id),
        name: name.to_string(),
        account_type: AccountType::from(account_type.to_string()),
        account_num,
        status: AccountStatus::from(status),
        notes,
        initial_balance: Money {
            v1: initial_balance.to_string(),
        },
        currency_id: CurrencyId::new(currency_id),
        favorite,
    };

    ctx.accounts().update_account(&account)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Account {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    name: Option<String>,
    account_type: Option<String>,
    initial_balance: Option<String>,
    currency_id: Option<i64>,
    account_num: Option<String>,
    status: Option<String>,
    notes: Option<String>,
    favorite: Option<bool>,
    format: OutputFormat,
) -> Result<()> {
    let update = AccountUpdate {
        name,
        account_type: account_type.map(AccountType::from),
        account_num,
        status: status.map(AccountStatus::from),
        notes,
        initial_balance: initial_balance.map(|s| Money { v1: s }),
        currency_id: currency_id.map(CurrencyId::new),
        favorite,
    };

    ctx.accounts()
        .update_account_partial(AccountId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Account {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.accounts().delete_account(AccountId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Account {} deleted successfully", id);
        }
    }
    Ok(())
}
