use crate::cli::CurrencyCommands;
use crate::output::{format_option, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::currencies::{Currency, CurrencyUpdate};
use mmex_lib::domain::types::{CurrencyId, Money};
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &CurrencyCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        CurrencyCommands::List => list(ctx, format),
        CurrencyCommands::Get { id } => get(ctx, *id, format),
        CurrencyCommands::BySymbol { symbol } => by_symbol(ctx, symbol, format),
        CurrencyCommands::Create {
            name,
            symbol,
            currency_type,
            scale,
            base_conv_rate,
            pfx_symbol,
            sfx_symbol,
            decimal_point,
            group_separator,
            unit_name,
            cent_name,
        } => create(
            ctx,
            name,
            symbol,
            currency_type,
            *scale,
            base_conv_rate,
            pfx_symbol.clone(),
            sfx_symbol.clone(),
            decimal_point.clone(),
            group_separator.clone(),
            unit_name.clone(),
            cent_name.clone(),
            format,
        ),
        CurrencyCommands::Update {
            id,
            name,
            symbol,
            currency_type,
            scale,
            base_conv_rate,
            pfx_symbol,
            sfx_symbol,
            decimal_point,
            group_separator,
            unit_name,
            cent_name,
        } => update(
            ctx,
            *id,
            name,
            symbol,
            currency_type,
            *scale,
            base_conv_rate,
            pfx_symbol.clone(),
            sfx_symbol.clone(),
            decimal_point.clone(),
            group_separator.clone(),
            unit_name.clone(),
            cent_name.clone(),
            format,
        ),
        CurrencyCommands::UpdatePartial {
            id,
            name,
            symbol,
            currency_type,
            scale,
            base_conv_rate,
            pfx_symbol,
            sfx_symbol,
            decimal_point,
            group_separator,
            unit_name,
            cent_name,
        } => update_partial(
            ctx,
            *id,
            name.clone(),
            symbol.clone(),
            currency_type.clone(),
            *scale,
            base_conv_rate.clone(),
            pfx_symbol.clone(),
            sfx_symbol.clone(),
            decimal_point.clone(),
            group_separator.clone(),
            unit_name.clone(),
            cent_name.clone(),
            format,
        ),
        CurrencyCommands::Delete { id } => delete(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let currencies = ctx.currencies().get_all_currencies()?;

    match format {
        OutputFormat::Json => print_json(&currencies),
        OutputFormat::Table => {
            let headers = ["ID", "Name", "Symbol", "Type"];
            let rows: Vec<Vec<String>> = currencies
                .iter()
                .map(|c| {
                    vec![
                        c.id.v1.to_string(),
                        c.name.clone(),
                        c.symbol.clone(),
                        c.currency_type.clone(),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let currency = ctx
        .currencies()
        .get_currency_by_id(mmex_lib::domain::types::CurrencyId::new(id))?;

    match currency {
        Some(c) => match format {
            OutputFormat::Json => print_json(&c),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), c.id.v1.to_string()],
                    vec!["Name".to_string(), c.name.clone()],
                    vec!["Symbol".to_string(), c.symbol.clone()],
                    vec!["Type".to_string(), c.currency_type.clone()],
                    vec!["Prefix".to_string(), format_option(c.pfx_symbol)],
                    vec!["Suffix".to_string(), format_option(c.sfx_symbol)],
                    vec!["Decimal".to_string(), format_option(c.decimal_point)],
                    vec!["Group Sep".to_string(), format_option(c.group_separator)],
                    vec!["Unit Name".to_string(), format_option(c.unit_name)],
                    vec!["Cent Name".to_string(), format_option(c.cent_name)],
                    vec!["Scale".to_string(), c.scale.to_string()],
                    vec!["Base Rate".to_string(), c.base_conv_rate.v1.clone()],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Currency not found"),
    }
    Ok(())
}

fn by_symbol(ctx: &MmexContext, symbol: &str, format: OutputFormat) -> Result<()> {
    let currency = ctx.currencies().get_currency_by_symbol(symbol)?;

    match currency {
        Some(c) => match format {
            OutputFormat::Json => print_json(&c),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), c.id.v1.to_string()],
                    vec!["Name".to_string(), c.name.clone()],
                    vec!["Symbol".to_string(), c.symbol.clone()],
                    vec!["Type".to_string(), c.currency_type.clone()],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Currency not found for symbol: {}", symbol),
    }
    Ok(())
}

fn create(
    ctx: &MmexContext,
    name: &str,
    symbol: &str,
    currency_type: &str,
    scale: i32,
    base_conv_rate: &str,
    pfx_symbol: Option<String>,
    sfx_symbol: Option<String>,
    decimal_point: Option<String>,
    group_separator: Option<String>,
    unit_name: Option<String>,
    cent_name: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let currency = Currency {
        id: CurrencyId::new(0),
        name: name.to_string(),
        pfx_symbol,
        sfx_symbol,
        decimal_point,
        group_separator,
        unit_name,
        cent_name,
        scale,
        base_conv_rate: Money {
            v1: base_conv_rate.to_string(),
        },
        symbol: symbol.to_string(),
        currency_type: currency_type.to_string(),
    };

    let created = ctx.currencies().create_currency(&currency)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Currency created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(
    ctx: &MmexContext,
    id: i64,
    name: &str,
    symbol: &str,
    currency_type: &str,
    scale: i32,
    base_conv_rate: &str,
    pfx_symbol: Option<String>,
    sfx_symbol: Option<String>,
    decimal_point: Option<String>,
    group_separator: Option<String>,
    unit_name: Option<String>,
    cent_name: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let currency = Currency {
        id: CurrencyId::new(id),
        name: name.to_string(),
        pfx_symbol,
        sfx_symbol,
        decimal_point,
        group_separator,
        unit_name,
        cent_name,
        scale,
        base_conv_rate: Money {
            v1: base_conv_rate.to_string(),
        },
        symbol: symbol.to_string(),
        currency_type: currency_type.to_string(),
    };

    ctx.currencies().update_currency(&currency)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Currency {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    name: Option<String>,
    symbol: Option<String>,
    currency_type: Option<String>,
    scale: Option<i32>,
    base_conv_rate: Option<String>,
    pfx_symbol: Option<String>,
    sfx_symbol: Option<String>,
    decimal_point: Option<String>,
    group_separator: Option<String>,
    unit_name: Option<String>,
    cent_name: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let update = CurrencyUpdate {
        name,
        pfx_symbol,
        sfx_symbol,
        decimal_point,
        group_separator,
        unit_name,
        cent_name,
        scale,
        base_conv_rate: base_conv_rate.map(|v| Money { v1: v }),
        symbol,
        currency_type,
    };

    ctx.currencies()
        .update_currency_partial(CurrencyId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Currency {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.currencies().delete_currency(CurrencyId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Currency {} deleted successfully", id);
        }
    }
    Ok(())
}
