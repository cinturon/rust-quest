use crate::quest::game::ActiveQuest;
use crate::quest::game::WORKSPACE_DIR;
use crate::quest::game::load_quest_pack;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::fs::write;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub completed_quest_ids: HashSet<String>,
    pub xp: i32,
    #[serde(default)]
    pub titles: Vec<String>,
    #[serde(default)]
    pub level: Level,
    #[serde(default)]
    pub zone_progress: Vec<ZoneInfo>,
}

#[derive(Serialize, Deserialize, Default,PartialEq)]
pub enum Level {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    Master,
    Grandmaster,
    Legendary,
    Mythic,
    Transcendent,
    Godlike,
}

pub enum Title {
    InitiateCoder,
    RustWanderer,
    ForestOfOwnership,
    VillageOfVariables,
}

impl Title {
    pub fn get_title(&self) -> String {
        match self {
            Title::InitiateCoder => "Initiate Coder".to_string(),
            Title::RustWanderer => "Rust Wanderer".to_string(),
            Title::ForestOfOwnership => "Forest Ranger".to_string(),
            Title::VillageOfVariables => "Village Graduate".to_string(),
        }
    }
}

impl Level {
    pub fn as_str(&self) -> &str {
        match self {
            Level::Beginner => "Beginner",
            Level::Intermediate => "Intermediate",
            Level::Advanced => "Advanced",
            Level::Expert => "Expert",
            Level::Master => "Master",
            Level::Grandmaster => "Grandmaster",
            Level::Legendary => "Legendary",
            Level::Mythic => "Mythic",
            Level::Transcendent => "Transcendent",
            Level::Godlike => "Godlike",
        }
    }
}

pub fn create_profile_json() -> Result<(), anyhow::Error> {
    let profile_json_file = PathBuf::from(WORKSPACE_DIR)
        .parent()
        .ok_or(anyhow::anyhow!("Failed to get parent directory"))?
        .join("profile.json");

    if profile_json_file.exists() {
        return Ok(());
    }

    let profile = Profile {
        completed_quest_ids: HashSet::new(),
        xp: 0,
        titles: Vec::new(),
        level: Level::Beginner,
        zone_progress: Vec::new(),
    };
    write(&profile_json_file, serde_json::to_string_pretty(&profile)?)?;
    Ok(())
}

pub fn save_progress_to_profile_json(active_quest: &ActiveQuest) -> Result<(), anyhow::Error> {
    let profile_json_file = PathBuf::from(WORKSPACE_DIR)
        .parent()
        .ok_or(anyhow::anyhow!("Failed to get parent directory"))?
        .join("profile.json");

    if profile_json_file.exists() {
        let mut profile: Profile =
            serde_json::from_reader(BufReader::new(File::open(&profile_json_file)?))?;

        if !profile.completed_quest_ids.contains(&active_quest.quest_id) && active_quest.completed {
            profile
                .completed_quest_ids
                .insert(active_quest.quest_id.clone());
            profile.xp += active_quest.xp;
            
            let level = get_level(profile.xp);
            if level != profile.level {
                eprintln!("Level Up! You are now a {}!", level.as_str());
            }
            profile.level = level;

            zone_progress(&mut profile)?;
            get_titles(&mut profile)?;

            write(&profile_json_file, serde_json::to_string_pretty(&profile)?)?;
            return Ok(());
        }

        Ok(())
    } else {
        let mut completed_quest_ids = HashSet::new();
        completed_quest_ids.insert(active_quest.quest_id.clone());
        let mut profile = Profile {
            completed_quest_ids,
            xp: active_quest.xp,
            titles: Vec::new(),
            level: get_level(active_quest.xp),
            zone_progress: Vec::new(),
        };
        zone_progress(&mut profile)?;
        get_titles(&mut profile)?;
        
        write(&profile_json_file, serde_json::to_string_pretty(&profile)?)?;
        Ok(())
    }
}

pub fn load_profile() -> Result<Profile, anyhow::Error> {
    // Create profile.json if it doesn't exist
    let profile_json_file = PathBuf::from(WORKSPACE_DIR)
        .parent()
        .ok_or(anyhow::anyhow!("Failed to get parent directory"))?
        .join("profile.json");
    let mut profile: Profile =
        serde_json::from_reader(BufReader::new(File::open(&profile_json_file)?))?;
    profile.level = get_level(profile.xp);
    Ok(profile)
}

pub fn get_level(xp: i32) -> Level {
    match xp {
        xp if xp < 10 => Level::Beginner,
        xp if xp < 25 => Level::Intermediate,
        xp if xp < 45 => Level::Advanced,
        xp if xp < 75 => Level::Expert,
        xp if xp < 100 => Level::Master,
        xp if xp < 130 => Level::Grandmaster,
        xp if xp < 165 => Level::Legendary,
        xp if xp < 200 => Level::Mythic,
        xp if xp < 235 => Level::Transcendent,
        xp if xp >= 235 => Level::Godlike,
        _ => Level::Beginner,
    }
}

#[derive(Serialize, Deserialize)]
pub struct ZoneInfo {
    pub name: String,
    pub completed_quests: usize,
    pub total_quests: usize,
}

impl ZoneInfo {
    pub fn new(name: String, completed_quests: usize, total_quests: usize) -> Self {
        Self {
            name,
            completed_quests,
            total_quests,
        }
    }
}
pub fn zone_progress(profile: &mut Profile) -> Result<(), anyhow::Error> {
    let quest_pack = load_quest_pack()?;

    // Clear previous zone progress
    profile.zone_progress.clear();

    let zones: HashSet<String> = quest_pack
        .quests
        .iter()
        .map(|quest| quest.zone.clone())
        .collect();

    for zone in zones {
        let completed_quests = quest_pack
            .quests
            .iter()
            .filter(|quest| quest.zone == zone && profile.completed_quest_ids.contains(&quest.id))
            .count();

        let total_quests = quest_pack
            .quests
            .iter()
            .filter(|quest| quest.zone == zone)
            .count();
        
        profile.zone_progress.push(ZoneInfo::new(zone, completed_quests, total_quests));
    }

    Ok(())
}

pub fn get_titles(profile: &mut Profile) -> Result<(), anyhow::Error> {
    if !profile.completed_quest_ids.is_empty()
        && !profile.titles.contains(&Title::InitiateCoder.get_title())
    {
        profile.titles.push(Title::InitiateCoder.get_title());
        eprintln!("Title Earned: {}", Title::InitiateCoder.get_title());
    }

    if profile.xp >= 50 && !profile.titles.contains(&Title::RustWanderer.get_title()) {
        profile.titles.push(Title::RustWanderer.get_title());
        eprintln!("Title Earned: {}", Title::RustWanderer.get_title());
    }


    if profile.zone_progress.iter().any(|zone| {
        zone.completed_quests == zone.total_quests && zone.name == "Forest of Ownership"
    }) && !profile
        .titles
        .contains(&Title::ForestOfOwnership.get_title())
        
    {
        profile.titles.push(Title::ForestOfOwnership.get_title());
        eprintln!("Title Earned: {}", Title::ForestOfOwnership.get_title());
    }

    if profile.zone_progress.iter().any(|zone| {
        zone.completed_quests == zone.total_quests && zone.name == "Village of Variables"
    }) && !profile
        .titles
        .contains(&Title::VillageOfVariables.get_title())
    {
        profile.titles.push(Title::VillageOfVariables.get_title());
        eprintln!("Title Earned: {}", Title::VillageOfVariables.get_title());
    }

    Ok(())
}
