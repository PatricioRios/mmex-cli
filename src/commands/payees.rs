use crate::cli::PayeeCommands;
use crate::output::{format_bool, format_option, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::payees::{Payee, PayeeUpdate};
use mmex_lib::domain::types::PayeeId;
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &PayeeCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        PayeeCommands::List => list(ctx, format),
        PayeeCommands::Get { id } => get(ctx, *id, format),
        PayeeCommands::Create { name } => create(ctx, name, format),
        PayeeCommands::Update {
            id,
            name,
            active,
            category_id,
            number,
            website,
            notes,
            pattern,
        } => update(
            ctx,
            *id,
            name,
            *active,
            *category_id,
            number.clone(),
            website.clone(),
            notes.clone(),
            pattern.clone(),
            format,
        ),
        PayeeCommands::UpdatePartial {
            id,
            name,
            active,
            category_id,
            number,
            website,
            notes,
            pattern,
        } => update_partial(
            ctx,
            *id,
            name.clone(),
            *active,
            *category_id,
            number.clone(),
            website.clone(),
            notes.clone(),
            pattern.clone(),
            format,
        ),
        PayeeCommands::Delete { id } => delete(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let payees = ctx.payees().get_all_payees()?;

    match format {
        OutputFormat::Json => print_json(&payees),
        OutputFormat::Table => {
            let headers = ["ID", "Name", "Active", "Category"];
            let rows: Vec<Vec<String>> = payees
                .iter()
                .map(|p| {
                    vec![
                        p.id.v1.to_string(),
                        p.name.clone(),
                        format_bool(p.active).to_string(),
                        p.category_id.map(|c| c.to_string()).unwrap_or_default(),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let payee = ctx
        .payees()
        .get_payee_by_id(mmex_lib::domain::types::PayeeId::new(id))?;

    match payee {
        Some(p) => match format {
            OutputFormat::Json => print_json(&p),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), p.id.v1.to_string()],
                    vec!["Name".to_string(), p.name.clone()],
                    vec!["Active".to_string(), format_bool(p.active).to_string()],
                    vec!["Category ID".to_string(), format_option(p.category_id)],
                    vec!["Number".to_string(), format_option(p.number)],
                    vec!["Website".to_string(), format_option(p.website)],
                    vec!["Notes".to_string(), format_option(p.notes)],
                    vec!["Pattern".to_string(), format_option(p.pattern)],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Payee not found"),
    }
    Ok(())
}

fn create(ctx: &MmexContext, name: &str, format: OutputFormat) -> Result<()> {
    let created = ctx.payees().create_payee(name)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Payee created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(
    ctx: &MmexContext,
    id: i64,
    name: &str,
    active: bool,
    category_id: Option<i64>,
    number: Option<String>,
    website: Option<String>,
    notes: Option<String>,
    pattern: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let payee = Payee {
        id: PayeeId::new(id),
        name: name.to_string(),
        category_id,
        number,
        website,
        notes,
        active,
        pattern,
    };

    ctx.payees().update_payee(&payee)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Payee {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    name: Option<String>,
    active: Option<bool>,
    category_id: Option<i64>,
    number: Option<String>,
    website: Option<String>,
    notes: Option<String>,
    pattern: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let update = PayeeUpdate {
        name,
        active,
        category_id,
        number,
        website,
        notes,
        pattern,
    };

    ctx.payees()
        .update_payee_partial(PayeeId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Payee {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.payees().delete_payee(PayeeId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Payee {} deleted successfully", id);
        }
    }
    Ok(())
}
