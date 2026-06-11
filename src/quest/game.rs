use crate::utils::util::sanitize_string;
use anyhow::Ok;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::{File, create_dir_all, write};
use std::io::BufReader;
use std::path::PathBuf;
use crate::quest::test::Test;
use crate::quest::test::run_test;


pub const WORKSPACE_DIR: &str = "./.cargo-quest/workspace";
const QUESTS_DIR: &str = "./src/quests";
const WORKSPACE_CARGO_TOML: &str = r#"version = "0.1.0"
edition = "2024"
"#;

#[derive(Serialize, Deserialize)]
pub struct QuestPack {
    pub quests: Vec<Quest>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub zone: String,
    pub instructions: String,
    pub xp: i32,
    pub starter: String,
    pub verify: Vec<Test>,
    pub victory_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ActiveQuest {
    pub title: String,
    pub quest_id: String,
    pub xp: i32,
    pub completed: bool,
    pub verify: Vec<Test>,
    #[serde(default)]
    pub victory_message: String,
}

pub fn load_quest(quest_id: &str) -> Result<Quest, anyhow::Error> {
    let path = PathBuf::from(QUESTS_DIR);
    let file = path.join(format!("{}.yaml", quest_id));
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let quest: Quest = serde_yaml::from_reader(reader)?;
    Ok(quest)
}

pub fn load_quest_pack() -> Result<QuestPack, anyhow::Error> {
    let path: PathBuf = PathBuf::from(QUESTS_DIR);

    let mut quests = Vec::new();
    for entry in path.read_dir()? {
        let path = entry?.path();

        // Check if entry is a file and a YAML file
        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        // Get the quest id from the file name
        let Some(quest_id) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        // Load the quest from the file
        let quest = load_quest(quest_id)?;

        quests.push(quest);
    }

    // Sort quests by id alphabetically
    quests.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(QuestPack { quests })
}

pub fn load_active_quest() -> Result<Option<ActiveQuest>, anyhow::Error> {
    let path = PathBuf::from(WORKSPACE_DIR)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory"))?
        .join("active.json");

    if !path.exists() {
        return Ok(None);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let active_quest: ActiveQuest = serde_json::from_reader(reader)?;
    Ok(Some(active_quest))
}

pub fn create_workspace_dir() -> Result<(), anyhow::Error> {
    let path: PathBuf = PathBuf::from(WORKSPACE_DIR);

    // Create workspace if it doesn't exist
    if !path.exists() {
        create_dir_all(path.join("src"))?;
        create_dir_all(path.join("tests"))?;
    }

    Ok(())
}

pub fn create_cargo_toml(quest: &Quest) -> Result<(), anyhow::Error> {
    let path = PathBuf::from(WORKSPACE_DIR);

    let toml_file = path.join("Cargo.toml");

    let cargo_toml = format!(
        "[package]\nname = \"{}\"\n{}",
        sanitize_string(&quest.title),
        WORKSPACE_CARGO_TOML
    );

    write(&toml_file, cargo_toml)?;

    Ok(())
}

pub fn create_main_rs(quest: &Quest) -> Result<(), anyhow::Error> {
    let path = PathBuf::from(WORKSPACE_DIR).join("src");

    let main_rs_file = path.join("main.rs");

    let mut content = String::new();
    
    content.push_str(&format!("// Quest: {}\n", quest.title));
    content.push('\n');
    content.push_str("// Instructions:\n");
    content.push('\n');
    
    for line in quest.instructions.lines() {
        content.push_str(&format!("// {}\n", line));
    }
    
    content.push('\n');
    content.push_str(&quest.starter);

    write(&main_rs_file, content)?;

    Ok(())
}

pub fn create_active_json(quest: &Quest) -> Result<(), anyhow::Error> {
    let active_json_file = PathBuf::from(WORKSPACE_DIR)
        .parent()
        .ok_or(anyhow::anyhow!("Failed to get parent directory"))?
        .join("active.json");

    let active = ActiveQuest {
        title: sanitize_string(&quest.title),
        quest_id: quest.id.clone(),
        xp: quest.xp,
        completed: false,
        verify: quest.verify.clone(),
        victory_message: quest.victory_message.clone(),
    };
   
    write(&active_json_file, serde_json::to_string_pretty(&active)?)?;

    Ok(())
}



pub fn save_active_json(active_quest: &ActiveQuest) -> Result<(), anyhow::Error> {
    let active_json_file = PathBuf::from(WORKSPACE_DIR)
        .parent()
        .ok_or(anyhow::anyhow!("Failed to get parent directory"))?
        .join("active.json");
    write(&active_json_file, serde_json::to_string_pretty(&active_quest)?)?;
    Ok(())
}

pub fn verify_workspace() -> Result<(), anyhow::Error> {
    let workspace_dir = PathBuf::from(WORKSPACE_DIR);
    if !workspace_dir.exists() {
        return Err(anyhow::anyhow!("Workspace directory not found"));
    }

    if !workspace_dir.join("src").exists() {
        return Err(anyhow::anyhow!("src directory not found"));
    }

    let cargo_toml = workspace_dir.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Err(anyhow::anyhow!("Cargo.toml file not found"));
    }

    let main_rs_file = workspace_dir.join("src/main.rs");
    if !main_rs_file.exists() {
        return Err(anyhow::anyhow!("main.rs file not found"));
    }

    let profile_json_file = workspace_dir.parent().ok_or(anyhow::anyhow!("Failed to get parent directory"))?.join("profile.json");
    if !profile_json_file.exists() {
        return Err(anyhow::anyhow!("profile.json file not found"));
    }

    let active_json_file = workspace_dir.parent().ok_or(anyhow::anyhow!("Failed to get parent directory"))?.join("active.json");
    if !active_json_file.exists() {
        return Err(anyhow::anyhow!("active.json file not found"));
    }
    
    Ok(())
}

pub fn verify_tests(active_quest: &ActiveQuest) -> Result<(), anyhow::Error> {
    for test in active_quest.verify.iter() {
        run_test(test)?;
    }
    Ok(())
}