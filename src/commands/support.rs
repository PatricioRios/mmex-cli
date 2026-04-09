use crate::cli::SupportCommands;
use crate::output::{print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &SupportCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        SupportCommands::DbVersion => db_version(ctx, format),
        SupportCommands::GetSetting { name } => get_setting(ctx, name, format),
        SupportCommands::SetSetting { name, value } => set_setting(ctx, name, value, format),
    }
}

fn db_version(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let version = ctx.support().get_db_version()?;

    match format {
        OutputFormat::Json => print_json(&json!({ "db_version": version })),
        OutputFormat::Table => {
            let headers = ["Field", "Value"];
            let rows = vec![vec!["DB Version".to_string(), version]];
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get_setting(ctx: &MmexContext, name: &str, format: OutputFormat) -> Result<()> {
    let value = ctx.support().get_setting(name)?;

    match value {
        Some(v) => match format {
            OutputFormat::Json => print_json(&json!({ name: v })),
            OutputFormat::Table => {
                let headers = ["Setting", "Value"];
                let rows = vec![vec![name.to_string(), v]];
                print_table(&headers, &rows);
            }
        },
        None => println!("Setting '{}' not found", name),
    }
    Ok(())
}

fn set_setting(ctx: &MmexContext, name: &str, value: &str, format: OutputFormat) -> Result<()> {
    ctx.support().set_setting(name, value)?;

    match format {
        OutputFormat::Json => {
            print_json(&json!({ "status": "success", "setting": name, "value": value }))
        }
        OutputFormat::Table => {
            println!("Successfully updated setting '{}' to '{}'", name, value);
        }
    }
    Ok(())
}
