use crate::cli::StockCommands;
use crate::output::{format_option, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::stocks::{Stock, StockUpdate};
use mmex_lib::domain::types::{MmexDate, Money, StockId};
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &StockCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        StockCommands::List => list(ctx, format),
        StockCommands::Get { id } => get(ctx, *id, format),
        StockCommands::Create {
            held_at,
            purchase_date,
            name,
            num_shares,
            purchase_price,
            current_price,
            value,
            commission,
            symbol,
            notes,
        } => create(
            ctx,
            *held_at,
            purchase_date,
            name,
            num_shares,
            purchase_price,
            current_price,
            value,
            commission,
            symbol.clone(),
            notes.clone(),
            format,
        ),
        StockCommands::Update {
            id,
            held_at,
            purchase_date,
            name,
            num_shares,
            purchase_price,
            current_price,
            value,
            commission,
            symbol,
            notes,
        } => update(
            ctx,
            *id,
            *held_at,
            purchase_date,
            name,
            num_shares,
            purchase_price,
            current_price,
            value,
            commission,
            symbol.clone(),
            notes.clone(),
            format,
        ),
        StockCommands::UpdatePartial {
            id,
            held_at,
            purchase_date,
            name,
            num_shares,
            purchase_price,
            current_price,
            value,
            commission,
            symbol,
            notes,
        } => update_partial(
            ctx,
            *id,
            *held_at,
            purchase_date.clone(),
            name.clone(),
            num_shares.clone(),
            purchase_price.clone(),
            current_price.clone(),
            value.clone(),
            commission.clone(),
            symbol.clone(),
            notes.clone(),
            format,
        ),
        StockCommands::Delete { id } => delete(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let stocks = ctx.stocks().get_all_stocks()?;

    match format {
        OutputFormat::Json => print_json(&stocks),
        OutputFormat::Table => {
            let headers = ["ID", "Name", "Symbol", "Shares", "Value"];
            let rows: Vec<Vec<String>> = stocks
                .iter()
                .map(|s| {
                    vec![
                        s.id.v1.to_string(),
                        s.name.clone(),
                        format_option(s.symbol.clone()),
                        s.num_shares.v1.clone(),
                        s.value.v1.clone(),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let stock = ctx
        .stocks()
        .get_stock_by_id(mmex_lib::domain::types::StockId::new(id))?;

    match stock {
        Some(s) => match format {
            OutputFormat::Json => print_json(&s),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), s.id.v1.to_string()],
                    vec!["Name".to_string(), s.name.clone()],
                    vec!["Symbol".to_string(), format_option(s.symbol)],
                    vec!["Held At (Account)".to_string(), s.held_at.to_string()],
                    vec!["Purchase Date".to_string(), s.purchase_date.v1.clone()],
                    vec!["Shares".to_string(), s.num_shares.v1.clone()],
                    vec!["Purchase Price".to_string(), s.purchase_price.v1.clone()],
                    vec!["Current Price".to_string(), s.current_price.v1.clone()],
                    vec!["Value".to_string(), s.value.v1.clone()],
                    vec!["Commission".to_string(), s.commission.v1.clone()],
                    vec!["Notes".to_string(), format_option(s.notes)],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Stock not found"),
    }
    Ok(())
}

fn create(
    ctx: &MmexContext,
    held_at: i64,
    purchase_date: &str,
    name: &str,
    num_shares: &str,
    purchase_price: &str,
    current_price: &str,
    value: &str,
    commission: &str,
    symbol: Option<String>,
    notes: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let stock = Stock {
        id: StockId::new(0),
        held_at,
        purchase_date: MmexDate {
            v1: purchase_date.to_string(),
        },
        name: name.to_string(),
        symbol,
        num_shares: Money {
            v1: num_shares.to_string(),
        },
        purchase_price: Money {
            v1: purchase_price.to_string(),
        },
        notes,
        current_price: Money {
            v1: current_price.to_string(),
        },
        value: Money {
            v1: value.to_string(),
        },
        commission: Money {
            v1: commission.to_string(),
        },
    };

    let created = ctx.stocks().create_stock(&stock)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Stock created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(
    ctx: &MmexContext,
    id: i64,
    held_at: i64,
    purchase_date: &str,
    name: &str,
    num_shares: &str,
    purchase_price: &str,
    current_price: &str,
    value: &str,
    commission: &str,
    symbol: Option<String>,
    notes: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let stock = Stock {
        id: StockId::new(id),
        held_at,
        purchase_date: MmexDate {
            v1: purchase_date.to_string(),
        },
        name: name.to_string(),
        symbol,
        num_shares: Money {
            v1: num_shares.to_string(),
        },
        purchase_price: Money {
            v1: purchase_price.to_string(),
        },
        notes,
        current_price: Money {
            v1: current_price.to_string(),
        },
        value: Money {
            v1: value.to_string(),
        },
        commission: Money {
            v1: commission.to_string(),
        },
    };

    ctx.stocks().update_stock(&stock)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Stock {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    held_at: Option<i64>,
    purchase_date: Option<String>,
    name: Option<String>,
    num_shares: Option<String>,
    purchase_price: Option<String>,
    current_price: Option<String>,
    value: Option<String>,
    commission: Option<String>,
    symbol: Option<String>,
    notes: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let update = StockUpdate {
        held_at,
        purchase_date: purchase_date.map(|s| MmexDate { v1: s }),
        name,
        symbol,
        num_shares: num_shares.map(|s| Money { v1: s }),
        purchase_price: purchase_price.map(|s| Money { v1: s }),
        notes,
        current_price: current_price.map(|s| Money { v1: s }),
        value: value.map(|s| Money { v1: s }),
        commission: commission.map(|s| Money { v1: s }),
    };

    ctx.stocks()
        .update_stock_partial(StockId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Stock {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.stocks().delete_stock(StockId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Stock {} deleted successfully", id);
        }
    }
    Ok(())
}
