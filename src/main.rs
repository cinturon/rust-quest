mod cli;
mod quest;
mod utils;


use anyhow::{Ok, Result};
use clap::Parser;
use cli::{Cli, Commands};
use quest::game::{load_quest, WORKSPACE_DIR};
use std::path::Path;

use crate::quest::game::{
    create_active_json, create_cargo_toml, create_main_rs, create_workspace_dir, load_quest_pack
};

fn main() -> Result<()> {

    // Parse CLI
    let cli = Cli::parse();

    // Execute CLI command
    match cli.command {
        Commands::List => cmd_list(),
        Commands::Start { quest_id } => cmd_start(&quest_id),
        Commands::Verify => cmd_verify(),
        Commands::Profile => cmd_profile(),
        Commands::QuestDetails { quest_id } => cmd_quest_details(&quest_id),
    }
}

fn cmd_list() -> Result<()> {
    let quest_pack = load_quest_pack()?;

    for quest in quest_pack.quests.iter() {
        eprintln!("id: {}", quest.id);
        eprintln!("title: {}", quest.title);
        eprintln!("zone: {}", quest.zone)
    }

    Ok(())
}

fn cmd_start(quest_id: &str) -> Result<()> {
    let quest = load_quest(quest_id)?;

    let workspace_dir = Path::new(WORKSPACE_DIR);
    
    if !workspace_dir.exists() {
        // Create workspace directory
        create_workspace_dir()?;
    }

    
        // Create main.rs
        create_main_rs(&quest)?;
    

    
        // Create Cargo.toml
        create_cargo_toml(&quest)?;
    

    
        // Create active.json
        create_active_json(&quest)?;
   

    // Print next steps
    eprintln!("Workspace created successfully for quest: {}", quest.id);
    eprintln!("Next steps:");
    eprintln!("1. Edit the code in the workspace");
    eprintln!("2. Run `cargo check` to verify your code");
    eprintln!("3. Run `cargo run` to run your code");
    eprintln!("4. Run `cargo-quest quest-details` to see the quest details");
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

fn cmd_quest_details(quest_id: &str) -> Result<()> {
    let quest = load_quest(quest_id)?;

    eprintln!(
        "id: {}\ntitle: {}\ninstructions: {} ",
        quest.id, quest.title, quest.instructions
    );
    Ok(())
}
