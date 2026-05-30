use serde::{Serialize, Deserialize};
use serde_yaml;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;

const QUESTS_DIR: &str  = "./src/quests";

#[derive(Serialize, Deserialize)]
pub struct Quest{
    pub id: String,
    pub title: String,
    pub zone: String,
    pub instructions: String,
    pub xp: i32,
    pub starter: String,
    pub verify: String
}

pub fn load_quest(quest_id: &str) -> Result<Quest, anyhow::Error>{
    let path = PathBuf::from(QUESTS_DIR);
    let file = path.join(format!("{}.yaml", quest_id));
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let quest: Quest = serde_yaml::from_reader(reader)?;
    Ok(quest)
}