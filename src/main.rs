mod cli;
mod commands;
mod output;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use mmex_lib::MmexContext;
use output::OutputFormat;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let format = OutputFormat::from_flag(args.json);

    match args.command {
        Commands::Version => {
            println!("mmex-cli version {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
            return Ok(());
        }
        _ => {}
    }

    let db_path = args.db.ok_or_else(|| {
        anyhow::anyhow!(
            "Database path is required. Use --db or set MMEX_DB_PATH environment variable."
        )
    })?;

    let ctx = MmexContext::open(Path::new(&db_path), args.key.clone())?;

    match args.command {
        Commands::Accounts(cmd) => commands::accounts::execute(&ctx, &cmd, format)?,
        Commands::Transactions(cmd) => commands::transactions::execute(&ctx, &cmd, format)?,
        Commands::Categories(cmd) => commands::categories::execute(&ctx, &cmd, format)?,
        Commands::Payees(cmd) => commands::payees::execute(&ctx, &cmd, format)?,
        Commands::Currencies(cmd) => commands::currencies::execute(&ctx, &cmd, format)?,
        Commands::Tags(cmd) => commands::tags::execute(&ctx, &cmd, format)?,
        Commands::Assets(cmd) => commands::assets::execute(&ctx, &cmd, format)?,
        Commands::Stocks(cmd) => commands::stocks::execute(&ctx, &cmd, format)?,
        Commands::Scheduled(cmd) => commands::scheduled::execute(&ctx, &cmd, format)?,
        Commands::Support(cmd) => commands::support::execute(&ctx, &cmd, format)?,
        Commands::Version | Commands::Completions { .. } => unreachable!(),
    }

    Ok(())
}
