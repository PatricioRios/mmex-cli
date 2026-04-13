use crate::cli::TransactionCommands;
use crate::output::{format_option, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::transactions::{
    SplitTransaction, Transaction, TransactionCode, TransactionStatus, TransactionUpdate,
};
use mmex_lib::domain::types::{
    AccountId, CategoryId, MmexDate, Money, PayeeId, TagId, TransactionId,
};
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &TransactionCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        TransactionCommands::List { account_id } => list(ctx, *account_id, format),
        TransactionCommands::Get { id } => get(ctx, *id, format),
        TransactionCommands::Create {
            account_id,
            payee_id,
            trans_code,
            amount,
            status,
            to_account_id,
            transaction_number,
            notes,
            category_id,
            date,
            to_amount,
        } => create(
            ctx,
            *account_id,
            *payee_id,
            trans_code,
            amount,
            status,
            *to_account_id,
            transaction_number.clone(),
            notes.clone(),
            *category_id,
            date.clone(),
            to_amount.clone(),
            format,
        ),
        TransactionCommands::Update {
            id,
            account_id,
            payee_id,
            trans_code,
            amount,
            status,
            to_account_id,
            transaction_number,
            notes,
            category_id,
            date,
            to_amount,
        } => update(
            ctx,
            *id,
            *account_id,
            *payee_id,
            trans_code,
            amount,
            status,
            *to_account_id,
            transaction_number.clone(),
            notes.clone(),
            *category_id,
            date.clone(),
            to_amount.clone(),
            format,
        ),
        TransactionCommands::UpdatePartial {
            id,
            account_id,
            payee_id,
            trans_code,
            amount,
            status,
            to_account_id,
            transaction_number,
            notes,
            category_id,
            date,
            to_amount,
        } => update_partial(
            ctx,
            *id,
            *account_id,
            *payee_id,
            trans_code.clone(),
            amount.clone(),
            status.clone(),
            *to_account_id,
            transaction_number.clone(),
            notes.clone(),
            *category_id,
            date.clone(),
            to_amount.clone(),
            format,
        ),
        TransactionCommands::Delete { id } => delete(ctx, *id, format),
        TransactionCommands::GetTags { id } => get_tags(ctx, *id, format),
        TransactionCommands::LinkTag { id, tag_id } => link_tag(ctx, *id, *tag_id, format),
        TransactionCommands::UnlinkTag { id, tag_id } => unlink_tag(ctx, *id, *tag_id, format),
        TransactionCommands::GetSplits { id } => get_splits(ctx, *id, format),
        TransactionCommands::AddSplit {
            transaction_id,
            amount,
            category_id,
            notes,
        } => add_split(
            ctx,
            *transaction_id,
            amount,
            *category_id,
            notes.clone(),
            format,
        ),
        TransactionCommands::UpdateSplit {
            id,
            transaction_id,
            amount,
            category_id,
            notes,
        } => update_split(
            ctx,
            *id,
            *transaction_id,
            amount,
            *category_id,
            notes.clone(),
            format,
        ),
        TransactionCommands::DeleteSplit { id } => delete_split(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, account_id: Option<i64>, format: OutputFormat) -> Result<()> {
    let transactions = match account_id {
        Some(id) => ctx
            .transactions()
            .get_transactions_for_account(AccountId::new(id))?,
        None => ctx.transactions().get_all_transactions()?,
    };

    match format {
        OutputFormat::Json => print_json(&transactions),
        OutputFormat::Table => {
            let headers = ["ID", "Account", "Date", "Code", "Amount", "Status"];
            let rows: Vec<Vec<String>> = transactions
                .iter()
                .map(|t| {
                    vec![
                        t.id.v1.to_string(),
                        t.account_id.v1.to_string(),
                        format_option(t.date.as_ref().map(|d| d.v1.clone())),
                        t.trans_code.to_string(),
                        t.amount.v1.clone(),
                        t.status.to_string(),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let tx = ctx
        .transactions()
        .get_transaction_by_id(mmex_lib::domain::types::TransactionId::new(id))?;

    match tx {
        Some(t) => match format {
            OutputFormat::Json => print_json(&t),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), t.id.v1.to_string()],
                    vec!["Account ID".to_string(), t.account_id.v1.to_string()],
                    vec![
                        "To Account ID".to_string(),
                        format_option(t.to_account_id.map(|a| a.v1)),
                    ],
                    vec!["Payee ID".to_string(), t.payee_id.v1.to_string()],
                    vec!["Code".to_string(), t.trans_code.to_string()],
                    vec!["Amount".to_string(), t.amount.v1.clone()],
                    vec!["Status".to_string(), t.status.to_string()],
                    vec!["Trans #".to_string(), format_option(t.transaction_number)],
                    vec!["Notes".to_string(), format_option(t.notes)],
                    vec![
                        "Category ID".to_string(),
                        format_option(t.category_id.map(|c| c.v1)),
                    ],
                    vec!["Date".to_string(), format_option(t.date.map(|d| d.v1))],
                    vec![
                        "To Amount".to_string(),
                        format_option(t.to_amount.map(|a| a.v1)),
                    ],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Transaction not found"),
    }
    Ok(())
}

fn create(
    ctx: &MmexContext,
    account_id: i64,
    payee_id: i64,
    trans_code: &str,
    amount: &str,
    status: &str,
    to_account_id: Option<i64>,
    transaction_number: Option<String>,
    notes: Option<String>,
    category_id: Option<i64>,
    date: Option<String>,
    to_amount: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let tx = Transaction {
        id: TransactionId::new(0),
        account_id: AccountId::new(account_id),
        to_account_id: to_account_id.map(AccountId::new),
        payee_id: PayeeId::new(payee_id),
        trans_code: TransactionCode::from(trans_code.to_string()),
        amount: Money {
            v1: amount.to_string(),
        },
        status: TransactionStatus::from(status.to_string()),
        transaction_number,
        notes,
        category_id: category_id.map(CategoryId::new),
        date: date.map(|d| MmexDate { v1: d }),
        to_amount: to_amount.map(|a| Money { v1: a }),
    };

    let created = ctx.transactions().create_transaction(&tx)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Transaction created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(
    ctx: &MmexContext,
    id: i64,
    account_id: i64,
    payee_id: i64,
    trans_code: &str,
    amount: &str,
    status: &str,
    to_account_id: Option<i64>,
    transaction_number: Option<String>,
    notes: Option<String>,
    category_id: Option<i64>,
    date: Option<String>,
    to_amount: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let tx = Transaction {
        id: TransactionId::new(id),
        account_id: AccountId::new(account_id),
        to_account_id: to_account_id.map(AccountId::new),
        payee_id: PayeeId::new(payee_id),
        trans_code: TransactionCode::from(trans_code.to_string()),
        amount: Money {
            v1: amount.to_string(),
        },
        status: TransactionStatus::from(status.to_string()),
        transaction_number,
        notes,
        category_id: category_id.map(CategoryId::new),
        date: date.map(|d| MmexDate { v1: d }),
        to_amount: to_amount.map(|a| Money { v1: a }),
    };

    ctx.transactions().update_transaction(&tx)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Transaction {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    account_id: Option<i64>,
    payee_id: Option<i64>,
    trans_code: Option<String>,
    amount: Option<String>,
    status: Option<String>,
    to_account_id: Option<i64>,
    transaction_number: Option<String>,
    notes: Option<String>,
    category_id: Option<i64>,
    date: Option<String>,
    to_amount: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let update = TransactionUpdate {
        account_id: account_id.map(AccountId::new),
        to_account_id: to_account_id.map(AccountId::new),
        payee_id: payee_id.map(PayeeId::new),
        trans_code: trans_code.map(TransactionCode::from),
        amount: amount.map(|a| Money { v1: a }),
        status: status.map(TransactionStatus::from),
        transaction_number,
        notes,
        category_id: category_id.map(CategoryId::new),
        date: date.map(|d| MmexDate { v1: d }),
        to_amount: to_amount.map(|a| Money { v1: a }),
    };

    ctx.transactions()
        .update_transaction_partial(TransactionId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Transaction {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.transactions()
        .delete_transaction(TransactionId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Transaction {} deleted successfully", id);
        }
    }
    Ok(())
}

fn get_tags(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let tags = ctx
        .transactions()
        .get_tags_for_transaction(TransactionId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&tags),
        OutputFormat::Table => {
            let headers = ["ID", "Name"];
            let rows: Vec<Vec<String>> = tags
                .iter()
                .map(|t| vec![t.id.v1.to_string(), t.name.clone()])
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn link_tag(ctx: &MmexContext, id: i64, tag_id: i64, format: OutputFormat) -> Result<()> {
    ctx.transactions()
        .link_tag(TransactionId::new(id), TagId::new(tag_id))?;

    match format {
        OutputFormat::Json => {
            print_json(&json!({ "status": "success", "transaction_id": id, "tag_id": tag_id }))
        }
        OutputFormat::Table => {
            println!("Tag {} linked to transaction {} successfully", tag_id, id);
        }
    }
    Ok(())
}

fn unlink_tag(ctx: &MmexContext, id: i64, tag_id: i64, format: OutputFormat) -> Result<()> {
    ctx.transactions()
        .unlink_tag(TransactionId::new(id), TagId::new(tag_id))?;

    match format {
        OutputFormat::Json => {
            print_json(&json!({ "status": "success", "transaction_id": id, "tag_id": tag_id }))
        }
        OutputFormat::Table => {
            println!(
                "Tag {} unlinked from transaction {} successfully",
                tag_id, id
            );
        }
    }
    Ok(())
}

fn get_splits(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let splits = ctx
        .transactions()
        .get_splits_for_transaction(TransactionId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&splits),
        OutputFormat::Table => {
            let headers = ["ID", "Category ID", "Amount", "Notes"];
            let rows: Vec<Vec<String>> = splits
                .iter()
                .map(|s| {
                    vec![
                        s.id.to_string(),
                        format_option(s.category_id.map(|c| c.v1)),
                        s.amount.v1.clone(),
                        format_option(s.notes.clone()),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn add_split(
    ctx: &MmexContext,
    transaction_id: i64,
    amount: &str,
    category_id: Option<i64>,
    notes: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let split = SplitTransaction {
        id: 0,
        transaction_id: TransactionId::new(transaction_id),
        category_id: category_id.map(CategoryId::new),
        amount: Money {
            v1: amount.to_string(),
        },
        notes,
    };

    let created = ctx.transactions().add_split(&split)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Split added successfully with ID {}", created.id);
        }
    }
    Ok(())
}

fn update_split(
    ctx: &MmexContext,
    id: i64,
    transaction_id: i64,
    amount: &str,
    category_id: Option<i64>,
    notes: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let split = SplitTransaction {
        id,
        transaction_id: TransactionId::new(transaction_id),
        category_id: category_id.map(CategoryId::new),
        amount: Money {
            v1: amount.to_string(),
        },
        notes,
    };

    ctx.transactions().update_split(&split)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Split {} updated successfully", id);
        }
    }
    Ok(())
}

fn delete_split(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.transactions().delete_split(id)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Split {} deleted successfully", id);
        }
    }
    Ok(())
}
