use anyhow::Ok;
use serde::{Serialize, Deserialize};
use serde_yaml;
use std::path::{PathBuf};
use std::io::BufReader;
use std::fs::File;

const QUESTS_DIR: &str  = "./src/quests";


#[derive(Serialize, Deserialize)]
pub struct QuestPack{
    pub quests: Vec<Quest>
}

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

pub fn load_quest_pack() -> Result<QuestPack, anyhow::Error>{
    let path: PathBuf = PathBuf::from(QUESTS_DIR);


    let mut quests = Vec::new();
    for entry in path.read_dir()?{
        
        let path = entry?.path();

        // Check if entry is a file and a YAML file
        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("yaml"){
            continue;
        }

        let Some(quest_id) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        let quest = load_quest(quest_id)?;

        quests.push(quest);    
    }

    // Sort quests by id alphabetically
    quests.sort_by(|a,b| a.id.cmp(&b.id));

    Ok(QuestPack { quests })
}