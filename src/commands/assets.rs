use crate::cli::AssetCommands;
use crate::output::{format_option, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::assets::{Asset, AssetStatus, AssetUpdate};
use mmex_lib::domain::types::{AssetId, CurrencyId, MmexDate, Money};
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &AssetCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        AssetCommands::List => list(ctx, format),
        AssetCommands::Get { id } => get(ctx, *id, format),
        AssetCommands::Create {
            name,
            start_date,
            status,
            value,
            currency_id,
            value_change_mode,
            value_change,
            notes,
            value_change_rate,
            asset_type,
        } => create(
            ctx,
            name,
            start_date,
            status,
            value,
            *currency_id,
            value_change_mode.clone(),
            value_change.clone(),
            notes.clone(),
            *value_change_rate,
            asset_type.clone(),
            format,
        ),
        AssetCommands::Update {
            id,
            name,
            start_date,
            status,
            value,
            currency_id,
            value_change_mode,
            value_change,
            notes,
            value_change_rate,
            asset_type,
        } => update(
            ctx,
            *id,
            name,
            start_date,
            status,
            value,
            *currency_id,
            value_change_mode.clone(),
            value_change.clone(),
            notes.clone(),
            *value_change_rate,
            asset_type.clone(),
            format,
        ),
        AssetCommands::UpdatePartial {
            id,
            name,
            start_date,
            status,
            value,
            currency_id,
            value_change_mode,
            value_change,
            notes,
            value_change_rate,
            asset_type,
        } => update_partial(
            ctx,
            *id,
            name.clone(),
            start_date.clone(),
            status.clone(),
            value.clone(),
            *currency_id,
            value_change_mode.clone(),
            value_change.clone(),
            notes.clone(),
            *value_change_rate,
            asset_type.clone(),
            format,
        ),
        AssetCommands::Delete { id } => delete(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let assets = ctx.assets().get_all_assets()?;

    match format {
        OutputFormat::Json => print_json(&assets),
        OutputFormat::Table => {
            let headers = ["ID", "Name", "Value", "Status", "Type"];
            let rows: Vec<Vec<String>> = assets
                .iter()
                .map(|a| {
                    vec![
                        a.id.v1.to_string(),
                        a.name.clone(),
                        a.value.v1.clone(),
                        a.status.to_string(),
                        format_option(a.asset_type.clone()),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let asset = ctx
        .assets()
        .get_asset_by_id(mmex_lib::domain::types::AssetId::new(id))?;

    match asset {
        Some(a) => match format {
            OutputFormat::Json => print_json(&a),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), a.id.v1.to_string()],
                    vec!["Name".to_string(), a.name.clone()],
                    vec!["Value".to_string(), a.value.v1.clone()],
                    vec!["Status".to_string(), a.status.to_string()],
                    vec!["Type".to_string(), format_option(a.asset_type)],
                    vec!["Start Date".to_string(), a.start_date.v1.clone()],
                    vec![
                        "Currency ID".to_string(),
                        format_option(a.currency_id.map(|c| c.v1)),
                    ],
                    vec![
                        "Change Mode".to_string(),
                        format_option(a.value_change_mode),
                    ],
                    vec!["Change".to_string(), format_option(a.value_change)],
                    vec!["Change Rate".to_string(), a.value_change_rate.to_string()],
                    vec!["Notes".to_string(), format_option(a.notes)],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Asset not found"),
    }
    Ok(())
}

fn create(
    ctx: &MmexContext,
    name: &str,
    start_date: &str,
    status: &str,
    value: &str,
    currency_id: Option<i64>,
    value_change_mode: Option<String>,
    value_change: Option<String>,
    notes: Option<String>,
    value_change_rate: f64,
    asset_type: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let asset = Asset {
        id: AssetId::new(0),
        name: name.to_string(),
        start_date: MmexDate {
            v1: start_date.to_string(),
        },
        status: AssetStatus::from(status.to_string()),
        currency_id: currency_id.map(CurrencyId::new),
        value_change_mode,
        value: Money {
            v1: value.to_string(),
        },
        value_change,
        notes,
        value_change_rate,
        asset_type,
    };

    let created = ctx.assets().create_asset(&asset)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Asset created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(
    ctx: &MmexContext,
    id: i64,
    name: &str,
    start_date: &str,
    status: &str,
    value: &str,
    currency_id: Option<i64>,
    value_change_mode: Option<String>,
    value_change: Option<String>,
    notes: Option<String>,
    value_change_rate: f64,
    asset_type: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let asset = Asset {
        id: AssetId::new(id),
        name: name.to_string(),
        start_date: MmexDate {
            v1: start_date.to_string(),
        },
        status: AssetStatus::from(status.to_string()),
        currency_id: currency_id.map(CurrencyId::new),
        value_change_mode,
        value: Money {
            v1: value.to_string(),
        },
        value_change,
        notes,
        value_change_rate,
        asset_type,
    };

    ctx.assets().update_asset(&asset)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Asset {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    name: Option<String>,
    start_date: Option<String>,
    status: Option<String>,
    value: Option<String>,
    currency_id: Option<i64>,
    value_change_mode: Option<String>,
    value_change: Option<String>,
    notes: Option<String>,
    value_change_rate: Option<f64>,
    asset_type: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let update = AssetUpdate {
        name,
        start_date: start_date.map(|s| MmexDate { v1: s }),
        status: status.map(AssetStatus::from),
        currency_id: currency_id.map(CurrencyId::new),
        value_change_mode,
        value: value.map(|s| Money { v1: s }),
        value_change,
        notes,
        value_change_rate,
        asset_type,
    };

    ctx.assets()
        .update_asset_partial(AssetId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Asset {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.assets().delete_asset(AssetId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Asset {} deleted successfully", id);
        }
    }
    Ok(())
}
