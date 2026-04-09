use crate::cli::ScheduledCommands;
use crate::output::{format_option, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::categories::CategoryId;
use mmex_lib::domain::scheduled_transactions::{ScheduledTransaction, ScheduledUpdate};
use mmex_lib::domain::transactions::{TransactionCode, TransactionStatus};
use mmex_lib::domain::types::{AccountId, MmexDate, Money, PayeeId};
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &ScheduledCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        ScheduledCommands::List => list(ctx, format),
        ScheduledCommands::Get { id } => get(ctx, *id, format),
        ScheduledCommands::Create {
            account_id,
            payee_id,
            trans_code,
            amount,
            status,
            repeats,
            num_occurrences,
            to_account_id,
            transaction_number,
            notes,
            category_id,
            trans_date,
            next_occurrence_date,
            to_trans_amount,
        } => create(
            ctx,
            *account_id,
            *payee_id,
            trans_code,
            amount,
            status,
            *repeats,
            *num_occurrences,
            *to_account_id,
            transaction_number.clone(),
            notes.clone(),
            *category_id,
            trans_date.clone(),
            next_occurrence_date.clone(),
            to_trans_amount.clone(),
            format,
        ),
        ScheduledCommands::Update {
            id,
            account_id,
            payee_id,
            trans_code,
            amount,
            status,
            repeats,
            num_occurrences,
            to_account_id,
            transaction_number,
            notes,
            category_id,
            trans_date,
            next_occurrence_date,
            to_trans_amount,
        } => update(
            ctx,
            *id,
            *account_id,
            *payee_id,
            trans_code,
            amount,
            status,
            *repeats,
            *num_occurrences,
            *to_account_id,
            transaction_number.clone(),
            notes.clone(),
            *category_id,
            trans_date.clone(),
            next_occurrence_date.clone(),
            to_trans_amount.clone(),
            format,
        ),
        ScheduledCommands::UpdatePartial {
            id,
            account_id,
            payee_id,
            trans_code,
            amount,
            status,
            repeats,
            num_occurrences,
            to_account_id,
            transaction_number,
            notes,
            category_id,
            trans_date,
            next_occurrence_date,
            to_trans_amount,
        } => update_partial(
            ctx,
            *id,
            *account_id,
            *payee_id,
            trans_code.clone(),
            amount.clone(),
            status.clone(),
            *repeats,
            *num_occurrences,
            *to_account_id,
            transaction_number.clone(),
            notes.clone(),
            *category_id,
            trans_date.clone(),
            next_occurrence_date.clone(),
            to_trans_amount.clone(),
            format,
        ),
        ScheduledCommands::Delete { id } => delete(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let scheduled = ctx.scheduled().get_all_scheduled()?;

    match format {
        OutputFormat::Json => print_json(&scheduled),
        OutputFormat::Table => {
            let headers = ["ID", "Account", "Code", "Amount", "Next Date"];
            let rows: Vec<Vec<String>> = scheduled
                .iter()
                .map(|s| {
                    vec![
                        s.id.to_string(),
                        s.account_id.v1.to_string(),
                        s.trans_code.to_string(),
                        s.amount.v1.clone(),
                        format_option(s.next_occurrence_date.as_ref().map(|d| d.v1.clone())),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let scheduled = ctx.scheduled().get_scheduled_by_id(id)?;

    match scheduled {
        Some(s) => match format {
            OutputFormat::Json => print_json(&s),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), s.id.to_string()],
                    vec!["Account ID".to_string(), s.account_id.v1.to_string()],
                    vec![
                        "To Account ID".to_string(),
                        format_option(s.to_account_id.map(|a| a.v1)),
                    ],
                    vec!["Payee ID".to_string(), s.payee_id.v1.to_string()],
                    vec!["Code".to_string(), s.trans_code.to_string()],
                    vec!["Amount".to_string(), s.amount.v1.clone()],
                    vec!["Status".to_string(), s.status.to_string()],
                    vec!["Trans #".to_string(), format_option(s.transaction_number)],
                    vec!["Notes".to_string(), format_option(s.notes)],
                    vec![
                        "Category ID".to_string(),
                        format_option(s.category_id.map(|c| c.v1)),
                    ],
                    vec![
                        "Trans Date".to_string(),
                        format_option(s.trans_date.map(|d| d.v1)),
                    ],
                    vec![
                        "Next Occurrence".to_string(),
                        format_option(s.next_occurrence_date.map(|d| d.v1)),
                    ],
                    vec!["Repeats".to_string(), s.repeats.to_string()],
                    vec!["Num Occurrences".to_string(), s.num_occurrences.to_string()],
                    vec![
                        "To Amount".to_string(),
                        format_option(s.to_trans_amount.map(|a| a.v1)),
                    ],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Scheduled transaction not found"),
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
    repeats: i32,
    num_occurrences: i32,
    to_account_id: Option<i64>,
    transaction_number: Option<String>,
    notes: Option<String>,
    category_id: Option<i64>,
    trans_date: Option<String>,
    next_occurrence_date: Option<String>,
    to_trans_amount: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let tx = ScheduledTransaction {
        id: 0,
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
        trans_date: trans_date.map(|d| MmexDate { v1: d }),
        next_occurrence_date: next_occurrence_date.map(|d| MmexDate { v1: d }),
        repeats,
        num_occurrences,
        to_trans_amount: to_trans_amount.map(|a| Money { v1: a }),
    };

    let created = ctx.scheduled().create_scheduled(&tx)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!(
                "Scheduled transaction created successfully with ID {}",
                created.id
            );
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
    repeats: i32,
    num_occurrences: i32,
    to_account_id: Option<i64>,
    transaction_number: Option<String>,
    notes: Option<String>,
    category_id: Option<i64>,
    trans_date: Option<String>,
    next_occurrence_date: Option<String>,
    to_trans_amount: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let tx = ScheduledTransaction {
        id,
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
        trans_date: trans_date.map(|d| MmexDate { v1: d }),
        next_occurrence_date: next_occurrence_date.map(|d| MmexDate { v1: d }),
        repeats,
        num_occurrences,
        to_trans_amount: to_trans_amount.map(|a| Money { v1: a }),
    };

    ctx.scheduled().update_scheduled(&tx)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Scheduled transaction {} updated successfully", id);
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
    repeats: Option<i32>,
    num_occurrences: Option<i32>,
    to_account_id: Option<i64>,
    transaction_number: Option<String>,
    notes: Option<String>,
    category_id: Option<i64>,
    trans_date: Option<String>,
    next_occurrence_date: Option<String>,
    to_trans_amount: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let update = ScheduledUpdate {
        account_id: account_id.map(AccountId::new),
        to_account_id: to_account_id.map(AccountId::new),
        payee_id: payee_id.map(PayeeId::new),
        trans_code: trans_code.map(TransactionCode::from),
        amount: amount.map(|a| Money { v1: a }),
        status: status.map(TransactionStatus::from),
        transaction_number,
        notes,
        category_id: category_id.map(CategoryId::new),
        trans_date: trans_date.map(|d| MmexDate { v1: d }),
        next_occurrence_date: next_occurrence_date.map(|d| MmexDate { v1: d }),
        repeats,
        num_occurrences,
        to_trans_amount: to_trans_amount.map(|a| Money { v1: a }),
    };

    ctx.scheduled().update_scheduled_partial(id, update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!(
                "Scheduled transaction {} partially updated successfully",
                id
            );
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.scheduled().delete_scheduled(id)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Scheduled transaction {} deleted successfully", id);
        }
    }
    Ok(())
}
