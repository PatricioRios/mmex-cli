mod cli;
mod commands;
mod output;

use clap::Parser;
use cli::{Cli, Commands};
use mmex_lib::MmexContext;
use output::OutputFormat;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let format = OutputFormat::from_flag(args.json);

    let ctx = MmexContext::open(Path::new(&args.db), args.key.clone())?;


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
        Commands::Version => println!("mmex-cli version {}", env!("CARGO_PKG_VERSION")),
    }

    Ok(())
}
