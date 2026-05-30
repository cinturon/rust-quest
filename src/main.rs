mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => cmd_list(),
        Commands::Start { quest_id } => cmd_start(&quest_id),
        Commands::Verify => cmd_verify(),
        Commands::Profile => cmd_profile(),
    }
}

fn cmd_list() -> Result<()> {
    eprintln!("list: coming in Milestone 2");
    Ok(())
}

fn cmd_start(quest_id: &str) -> Result<()> {
    eprintln!("start {quest_id}: coming in Milestone 3");
    Ok(())
}

fn cmd_verify() -> Result<()> {
    eprintln!("verify: coming in Milestone 4");
    Ok(())
}

fn cmd_profile() -> Result<()> {
    eprintln!("profile: coming in Milestone 5");
    Ok(())
}
