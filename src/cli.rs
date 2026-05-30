use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cargo-quest",
    version,
    about = "Learn Rust through small coding quests",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List available quests
    List,
    /// Start a quest and generate an active workspace
    Start {
        /// Quest id (e.g. variables_001)
        quest_id: String,
    },
    /// Verify the active quest (compile and run checks)
    Verify,
    /// Show learner profile and XP
    Profile,
}
